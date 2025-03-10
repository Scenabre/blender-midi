use log::{error, info};
use midir::MidiOutputConnection;

use crate::midi_server::container::{
    BorrowedChannels, Event, RawMidi, Recipe, SIGflag, WaitingQueue,
};
use crate::midi_server::midi_event::craft_recipe;
use crate::midi_server::midi_process_mesg::{process_midi_mesg, CCflag};
use crate::midi_server::midi_send_mesg::{
    gen_lcd_string, initialize_mc_device, make_raw_midi_mesg, reset_mc_device,
};
use crate::midi_server::setup_client_params::{setup_client_params, AudioParams};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn init_midi_audio(
    tx: Sender<(u64, Vec<u8>)>,
    rx: Sender<(u64, Vec<u8>)>,
    ext_signal: Sender<bool>,
    int_signal: Arc<Mutex<bool>>,
    recipes: Arc<Mutex<Recipe>>,
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

            let init_mesgs: Vec<RawMidi> = initialize_mc_device().unwrap();
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
                        let mut recipe: (Vec<u8>, Vec<Vec<u8>>) = (vec![0x80, 0x18, 0x40], vec![]);
                        for mesg in mesgs {
                            recipe.1.push(mesg.data().to_vec());
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

            let wait_queue = Arc::new(Mutex::new(WaitingQueue::new()));
            let wait_queue_clone = wait_queue.clone();
            let current_lcd_mesg = Arc::new(Mutex::new("She bath'd with roses red, and violets blue, And all the sweetest flowres, that in the forrest grew.".to_string()));
            let current_lcd_mesg_clone = current_lcd_mesg.clone();
            let (int_tx, int_rx) = channel();
            let sig_flag = Arc::new(Mutex::new(SIGflag::default()));
            let cc_flag = Arc::new(Mutex::new(CCflag::default()));
            let lcd_last_value = Vec::<Vec<u8>>::new();

            let _conn_in = params.midi_input.connect(
                &params.midi_input_port,
                "bl-midi-in",
                move |stamp, message, midi_datas| {
                    input_callback(
                        &stamp,
                        message,
                        &midi_datas.2,
                        &midi_datas.3,
                        &midi_datas.4,
                        (&midi_datas.0, &midi_datas.1),
                        Some(&midi_datas.5),
                    );
                },
                (
                    rx,
                    tx,
                    sig_flag.clone(),
                    cc_flag.clone(),
                    int_tx.clone(),
                    triggers_events,
                    wait_queue.clone(),
                ),
            );

            let stop_signal_arc = Arc::clone(&int_signal);
            let mut loop_count_elapsed: u64 = 0;
            let duration: u64 = 1;

            loop {
                if *stop_signal_arc.lock().unwrap() {
                    return;
                }

                if let Ok(rx_data) = int_rx.try_recv() {
                    conn_out.send(&rx_data).unwrap();
                }

                if !wait_queue.lock().unwrap().is_empty() {
                    for (idx, (elaps, mesgs, is_send)) in
                        wait_queue.lock().unwrap().iter().enumerate()
                    {
                        if *elaps >= (loop_count_elapsed * duration) && !is_send {
                            for raw_midi in mesgs {
                                let _ = conn_out.send(raw_midi.data());
                            }
                        }
                    }
                }

                sleep(Duration::from_millis(duration));
                loop_count_elapsed += 1;
            }
        }
        Err(e) => error!("{}", e),
    };
}

fn input_callback(
    stamp: &u64,
    mesg: &[u8],
    sigflag: &Arc<Mutex<SIGflag>>,
    ccflag: &Arc<Mutex<CCflag>>,
    int_tx: &Sender<Vec<u8>>,
    ext_channels: BorrowedChannels,
    triggers: Option<&Vec<Event>>,
) {
    let raw_midi = RawMidi::new(*stamp, mesg).unwrap();
    let (ext_rx, ext_tx) = ext_channels;
    let mut sig_flag = sigflag.lock().unwrap();
    let mut cc_flag = ccflag.lock().unwrap();

    int_tx.send(mesg.to_vec()).unwrap();

    if mesg[0] == 0x80 && mesg[1] == 0x52 {
        if !sig_flag.reset_signal {
            sig_flag.reset_signal = true;
        }
    } else if sig_flag.reset_signal && mesg[0] == 0x90 && mesg[1] == 0x52 {
        match reset_mc_device() {
            Ok(to_send) => {
                for mesg in to_send.iter() {
                    let _ = int_tx.send(mesg.data().to_vec());
                    sleep(Duration::from_millis(10));
                }

                sleep(Duration::from_millis(100));
                let init_mesgs: Vec<RawMidi> = initialize_mc_device().unwrap();

                for mesg in init_mesgs.iter() {
                    let _ = int_tx.send(mesg.data().to_vec());
                    sleep(Duration::from_millis(10));
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
        Ok(mesgs) => match mesgs.to_send {
            Some(mesgs) => {
                for mesg in mesgs {
                    println!("Connection out found! Midi mesg : {:04X?}", mesg.data());

                    //match ext_tx.send(mesg.data().to_vec()) {
                    //    Ok(_) => (),
                    //    Err(err) => {
                    //        error!("Error when sending midi mesg to output : {}", err)
                    //    }
                    //}
                }
            }
            None => println!("Nothing to send, skip it…"),
        },
        Err(err) => println!("No midi mesg output : {}", err),
    };
}
