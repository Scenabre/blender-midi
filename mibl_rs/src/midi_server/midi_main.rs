use log::{error, info};
use midir::MidiOutputConnection;

use crate::midi_server::container::{
    Event, ExtTrigger, Ingredient, InitDevice, RawMidi, Recipe, SIGflag,
};
use crate::midi_server::midi_event::craft_recipe;
use crate::midi_server::midi_process_mesg::{process_midi_mesg, CCflag};
use crate::midi_server::midi_send_mesg::{
    gen_lcd_string, initialize_mc_device, reset_mc_device, send_note_bang,
};
use crate::midi_server::setup_client_params::setup_client_params;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn init_midi_audio(
    tx: Sender<Vec<ExtTrigger>>,
    ext_signal: Sender<bool>,
    int_signal: Arc<Mutex<bool>>,
    recipe: Arc<Mutex<Recipe>>,
    init_params: Arc<Mutex<InitDevice>>,
) {
    match setup_client_params() {
        Ok(params) => {
            println!("Connect to port : {}", params.port_name);

            let mut conn_out: MidiOutputConnection = match params
                .midi_output
                .connect(&params.midi_output_port, "bl-midi-out")
            {
                Ok(out) => out,
                Err(err) => {
                    error!("Unable to connect to output : {}", err);
                    return;
                }
            };

            sleep(Duration::from_millis(50));

            match init_params.lock() {
                Ok(init_params_lock) => {
                    let init_mesgs: Vec<RawMidi> = initialize_mc_device(&init_params_lock).unwrap();
                    info!("Sending all messages to midi device now…");

                    let init_mesgs_len = init_mesgs.len();

                    for (idx, mesg) in init_mesgs.iter().enumerate() {
                        info!(
                            "Sending mesg {}/{} : {:04X?}",
                            (idx + 1),
                            init_mesgs_len,
                            mesg.data()
                        );

                        let _ = conn_out.send(mesg.data());
                        sleep(Duration::from_millis(10));
                    }

                    let recipe_1: Option<Recipe> =
                        match gen_lcd_string(0, Some("This is a test !".to_string())) {
                            Ok(mesgs) => {
                                let mut recipe: Ingredient = (vec![0x80, 0x18, 0x40], vec![], None);
                                for mesg in mesgs {
                                    recipe.1.push(mesg.data().to_vec());
                                }
                                match send_note_bang(0x18, 0) {
                                    Ok(raw_midi) => {
                                        for mesg in raw_midi {
                                            recipe.1.push(mesg.data().to_vec());
                                        }
                                    }
                                    Err(err) => {
                                        println!(
                                            "Unable to generate note bang for recipe !  {}",
                                            err
                                        )
                                    }
                                }
                                Some(vec![recipe])
                            }
                            Err(err) => {
                                println!("Unable to create the recipe 1 : {}", err);
                                None
                            }
                        };

                    let triggers_events = match craft_recipe(&true, &recipe_1) {
                        Ok(events) => events,
                        Err(err) => panic!("Unable to create the trigger table ! {}", err),
                    };

                    println!("Triggers Events : {:?}", triggers_events);

                    info!("Initialization done!");

                    let (int_tx, int_rx) = channel();
                    let sig_flag = Arc::new(Mutex::new(SIGflag::default()));
                    let cc_flag = Arc::new(Mutex::new(CCflag::default()));
                    let init_params_clone = init_params.clone();

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
                                Some(&midi_datas.4),
                                &midi_datas.5,
                            );
                        },
                        (
                            sig_flag.clone(),
                            cc_flag.clone(),
                            int_tx.clone(),
                            tx,
                            triggers_events,
                            init_params_clone,
                        ),
                    );

                    let stop_signal_arc = Arc::clone(&int_signal);
                    let duration: u64 = 1;

                    loop {
                        if *stop_signal_arc.lock().unwrap() {
                            return;
                        }

                        if let Ok(rx_data) = int_rx.try_recv() {
                            for midi_data in rx_data {
                                conn_out.send(&midi_data).unwrap();
                            }
                        }

                        sleep(Duration::from_millis(duration));
                    }
                }
                Err(e) => error!("{}", e),
            };
        }
        Err(err) => println!("Unable to initialize device, continue : {}", err),
    }
}

fn input_callback(
    data_in: (&u64, &[u8]),
    sigflag: &Arc<Mutex<SIGflag>>,
    ccflag: &Arc<Mutex<CCflag>>,
    int_tx: &Sender<Vec<Vec<u8>>>,
    ext_tx: &Sender<Vec<ExtTrigger>>,
    triggers: Option<&Vec<Event>>,
    init_params: &Arc<Mutex<InitDevice>>,
) {
    let (stamp, mesg) = data_in;
    let raw_midi = RawMidi::new(*stamp, mesg).unwrap();
    let mut sig_flag = sigflag.lock().unwrap();
    let mut cc_flag = ccflag.lock().unwrap();

    int_tx.send(vec![mesg.to_vec()]).unwrap();

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
                match init_params.lock() {
                    Ok(init_params_lock) => {
                        let init_mesgs: Vec<RawMidi> =
                            initialize_mc_device(&init_params_lock).unwrap();

                        for mesg in init_mesgs.iter() {
                            let _ = int_tx.send(vec![mesg.data().to_vec()]);
                            sleep(Duration::from_millis(10));
                        }
                    }
                    Err(err) => println!(
                        "Unable to lock init parameters, abort Device initialization : {}",
                        err
                    ),
                }
            }
            Err(err) => println!("Unable to reset device : {}", err),
        }
        sig_flag.reset_signal = false;
    } else if sig_flag.reset_signal {
        sig_flag.reset_signal = false;
    }

    let midi_result = process_midi_mesg(&raw_midi, "MC", &mut cc_flag, triggers);

    match midi_result {
        Ok(mesgs) => {
            match mesgs.to_send.0 {
                Some(mesgs) => {
                    println!("Sending triggered data to midi port !");
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
                None => println!("Nothing to send to internal midi server !"),
            }

            match mesgs.to_send.1 {
                Some(mesgs) => {
                    println!("Preparing to send trigger to client : {:04X?}", mesgs);
                    match ext_tx.send(mesgs) {
                        Ok(_) => (),
                        Err(err) => {
                            error!("Error when sending midi mesg to output : {}", err)
                        }
                    }
                }
                None => println!("Nothing to send, skip it…"),
            }
        }
        Err(err) => println!("No midi mesg output : {}", err),
    };
}
