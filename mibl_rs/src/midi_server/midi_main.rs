use log::{error, info, warn};
use midir::MidiOutputConnection;

use crate::midi_server::container::{
    CCflag, DeviceState, Event, ExtTrigger, RawMidi, Recipe, SIGflag,
};
use crate::midi_server::midi_event::craft_recipe;
use crate::midi_server::midi_process_mesg::process_midi_mesg;
use crate::midi_server::midi_send_mesg::{
    gen_lcd_string, initialize_mc_device, reset_mc_device, send_note_bang, timestamp_gen,
};
use crate::midi_server::setup_client_params::setup_client_params;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn init_midi_audio(
    tx: Sender<Vec<ExtTrigger>>,
    ext_signal: Sender<DeviceState>,
    int_signal: Arc<Mutex<SIGflag>>,
    recipe: Arc<Mutex<Recipe>>,
    device_params: Arc<Mutex<DeviceState>>,
) {
    let debug = int_signal.lock().unwrap().debug;

    match setup_client_params() {
        Ok(params) => {
            if debug {
                println!("Connect to port : {}", params.port_name);
            }

            let mut conn_out: MidiOutputConnection = match params
                .midi_output
                .connect(&params.midi_output_port, "bl-midi-out")
            {
                Ok(out) => out,
                Err(err) => {
                    println!("Unable to connect to output : {}", err);
                    return;
                }
            };

            sleep(Duration::from_millis(50)); // Wait a little time after creating connection output

            match device_params.lock() {
                Ok(device_params_lock) => {
                    let init_mesgs = initialize_mc_device(&device_params_lock).unwrap();

                    if debug {
                        println!("Sending all messages to midi device now…");
                    }

                    let init_mesgs_len = init_mesgs.len();

                    for (idx, mesg) in init_mesgs.iter().enumerate() {
                        println!(
                            "Sending mesg {}/{} : {:04X?}",
                            (idx + 1),
                            init_mesgs_len,
                            mesg.data()
                        );

                        let _ = conn_out.send(mesg.data());
                        sleep(Duration::from_millis(10));
                    }
                }
                Err(err) => {
                    println!(
                        "Unable to access device state for initialization, closing thread : {}",
                        err
                    );
                    return;
                }
            }

            let triggers_events: Arc<Mutex<Option<Vec<Event>>>> = Arc::new(Mutex::new(None));

            let recipe_lock = recipe.lock().unwrap().clone();
            let mut opt_recipe = None;
            let use_sys_event = int_signal.lock().unwrap().use_sys_event;

            if !recipe_lock.is_empty() {
                opt_recipe = Some(&recipe_lock);
            }

            *triggers_events.lock().unwrap() = match craft_recipe(&use_sys_event, opt_recipe) {
                Ok(events) => {
                    println!("Build triggers before conn_in");
                    events
                }
                Err(err) => panic!("Unable to create the trigger table ! {}", err),
            };

            drop(recipe_lock);

            if debug {
                println!("Triggers Events : {:?}", triggers_events.lock().unwrap());

                println!("Initialization done!");
            }

            let (int_tx, int_rx) = channel();

            let _conn_in = params.midi_input.connect(
                &params.midi_input_port,
                "bl-midi-in",
                move |stamp, message, midi_datas| {
                    input_callback(
                        (&stamp, message),
                        &midi_datas.0,
                        &midi_datas.1,
                        &midi_datas.2,
                        &midi_datas.3,
                        &midi_datas.4,
                    );
                },
                (
                    int_signal.clone(),
                    int_tx.clone(),
                    tx,
                    triggers_events.clone(),
                    device_params.clone(),
                ),
            );

            let int_signal_arc = Arc::clone(&int_signal);
            let device_params_lock = device_params.lock().unwrap();
            let fps = *device_params_lock.get_fps();
            drop(device_params_lock);

            let duration: u64 = 1000 / fps;

            loop {
                if int_signal_arc.lock().unwrap().stop_thread {
                    return;
                }

                if int_signal_arc.lock().unwrap().update_recipe {
                    if debug {
                        println!("Updating recipe in loop");
                    }
                    let recipe = recipe.lock().unwrap().clone();
                    let mut opt_recipe = None;
                    let use_sys_event = int_signal.lock().unwrap().use_sys_event;

                    if !recipe.is_empty() {
                        opt_recipe = Some(&recipe);
                    }

                    *triggers_events.lock().unwrap() =
                        match craft_recipe(&use_sys_event, opt_recipe) {
                            Ok(events) => {
                                println!("Triggers build in loop");
                                events
                            }
                            Err(err) => panic!("Unable to create the trigger table ! {}", err),
                        };

                    int_signal_arc.lock().unwrap().update_recipe = false;
                }

                let timestamp = *device_params.lock().unwrap().get_timestamp();
                match timestamp_gen(timestamp[0], timestamp[1], timestamp[2], timestamp[3]) {
                    Ok(raw_timestamp) => {
                        for raw_midi in raw_timestamp {
                            conn_out.send(raw_midi.data()).unwrap();
                        }
                    }
                    Err(err) => println!("Unable to generate timestamp, continue… {}", err),
                }

                if let Ok(rx_data) = int_rx.try_recv() {
                    for midi_data in rx_data {
                        conn_out.send(&midi_data).unwrap();
                    }
                }

                sleep(Duration::from_millis(duration));
            }
        }
        Err(err) => println!("Unable to initialize device, continue : {}", err),
    }
}

fn input_callback(
    data_in: (&u64, &[u8]),
    sigflag: &Arc<Mutex<SIGflag>>,
    int_tx: &Sender<Vec<Vec<u8>>>,
    ext_tx: &Sender<Vec<ExtTrigger>>,
    triggers: &Arc<Mutex<Option<Vec<Event>>>>,
    device_params: &Arc<Mutex<DeviceState>>,
) {
    let (stamp, mesg) = data_in;
    let raw_midi = RawMidi::new(*stamp, mesg).unwrap();
    let mut sig_flag = sigflag.lock().unwrap();

    int_tx.send(vec![mesg.to_vec()]).unwrap();

    if mesg[0] == 0x90 {
        sig_flag.note_on = true;
        sig_flag.note_bang_value = mesg[1];
        sig_flag.note_bang = false;
    } else if mesg[0] == 0x80 && sig_flag.note_on && mesg[1] == sig_flag.note_bang_value {
        sig_flag.note_bang = true;
    } else {
        sig_flag.note_on = false;
        sig_flag.note_bang = false;
        sig_flag.note_bang_value = 0;
    }

    if mesg[0] == 0x80 && mesg[1] == 0x52 {
        if !sig_flag.reset_signal {
            sig_flag.reset_signal = true;
        }
    } else if sig_flag.reset_signal && mesg[0] == 0x90 && mesg[1] == 0x52 {
        match reset_mc_device() {
            Ok(to_send) => {
                match gen_lcd_string(0, None) {
                    Ok(raw_midi_mesgs) => {
                        let mut mesgs = vec![];
                        for mesg in raw_midi_mesgs {
                            mesgs.push(mesg.data().to_vec());
                        }
                        let _ = int_tx.send(mesgs);
                    }
                    Err(err) => println!("Unable to clear lcd string : {}", err),
                };

                match gen_lcd_string(0, Some("Reseting device !".to_string())) {
                    Ok(raw_midi_mesgs) => {
                        let mut mesgs = vec![];
                        for mesg in raw_midi_mesgs {
                            mesgs.push(mesg.data().to_vec());
                        }
                        let _ = int_tx.send(mesgs);
                    }
                    Err(err) => println!("Unable to generate reset info lcd string : {}", err),
                };

                for mesg in to_send.iter() {
                    let _ = int_tx.send(vec![mesg.data().to_vec()]);
                    sleep(Duration::from_millis(10));
                }

                sleep(Duration::from_millis(100));
                let mut init_mesgs: Vec<RawMidi> = vec![];

                match device_params.lock() {
                    Ok(device_params_lock) => {
                        init_mesgs = initialize_mc_device(&device_params_lock).unwrap()
                    }
                    Err(err) => println!("Unable to lock device state skip device reset : {}", err),
                }

                for mesg in init_mesgs.iter() {
                    let _ = int_tx.send(vec![mesg.data().to_vec()]);
                    sleep(Duration::from_millis(10));
                }
            }
            Err(err) => println!("Unable to reset device : {}", err),
        }
        sig_flag.reset_signal = false;
    } else if sig_flag.reset_signal {
        sig_flag.reset_signal = false;
    }

    let midi_result = process_midi_mesg(&raw_midi, "MC", &mut sig_flag, &triggers.lock().unwrap());

    match midi_result {
        Ok(mesgs) => {
            match mesgs.to_send.0 {
                Some(mesgs) => {
                    if sig_flag.debug {
                        println!("Sending triggered data to midi port !");
                    }
                    let mut midi_datas = vec![];

                    for midi_data in mesgs {
                        midi_datas.push(midi_data.data().to_vec());
                    }

                    match int_tx.send(midi_datas) {
                        Ok(_) => (),
                        Err(err) => {
                            println!("Unable to send mesg to Internal Sender : {}", err)
                        }
                    }
                }
                None => {
                    if sig_flag.debug {
                        println!("Nothing to send to internal midi server !")
                    }
                }
            }

            match mesgs.to_send.1 {
                Some(mesgs) => {
                    if sig_flag.debug {
                        println!("Preparing to send trigger to client : {:04X?}", mesgs);
                    }

                    match ext_tx.send(mesgs) {
                        Ok(_) => (),
                        Err(err) => {
                            println!("Error when sending midi mesg to output : {}", err)
                        }
                    }
                }
                None => {
                    if sig_flag.debug {
                        println!("Nothing to send, skip it…")
                    }
                }
            }
        }
        Err(err) => println!("No midi mesg output : {}", err),
    };
}
