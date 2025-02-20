use log::{error, info};
use midir::MidiOutputConnection;
use simple_logger::SimpleLogger;

use crate::midi_server::container::RawMidi;
use crate::midi_server::midi_process_mesg::{process_midi_mesg, CCflag};
use crate::midi_server::midi_send_mesg::initialize_mc_device;
use crate::midi_server::setup_client_params::setup_client_params;
use crate::MiBlRustProcess;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn init_midi_audio(
    tx: Sender<u64>,
    rx: Sender<u64>,
    ext_signal: Sender<bool>,
    int_signal: &mut Arc<Mutex<bool>>,
) {
    //SimpleLogger::new()
    //    .with_level(log::LevelFilter::Debug)
    //    .init()
    //    .unwrap();

    match setup_client_params() {
        Ok(mut params) => {
            let mut conn_out: Option<MidiOutputConnection> = match params
                .midi_output
                .connect(&params.midi_output_port, "bl-midi-out")
            {
                Ok(out) => Some(out),
                Err(err) => {
                    error!("Unable to connect to output : {}", err);
                    None
                }
            };

            let init_mesgs: Vec<RawMidi> = initialize_mc_device().unwrap();
            info!("Sending all messages to midi device now…");

            let init_mesgs_len = init_mesgs.len();

            for (idx, mesg) in init_mesgs.iter().enumerate() {
                //info!(
                //    "Sending mesg {}/{} : {:04X?}",
                //    (idx + 1),
                //    init_mesgs_len,
                //    mesg.data()
                //);
                let _ = conn_out.as_mut().unwrap().send(mesg.data());
                sleep(Duration::from_millis(10));
            }

            info!("Initialization done!");

            let _conn_in = params.midi_input.connect(
                &params.midi_input_port,
                "bl-midi-in",
                move |stamp, message, midi_datas| {
                    println!("IN test : {:?}, {:?}", stamp, message);
                    //let test = RawMidi::new(stamp, message).unwrap();
                    midi_datas.0.send(stamp).unwrap();

                    if midi_datas.3 == 100 {
                        println!("signal recv in connection in");
                        let _ = midi_datas.2.send(true);
                    }

                    midi_datas.3 += 1;
                    //input_callback(
                    //    &stamp,
                    //    message,
                    //    &mut params.cc_flag,
                    //    conn_out.as_mut(),
                    //    &midi_datas[0],
                    //    &midi_datas[1],
                    //);
                    //let midi_datas_locked = midi_datas.lock().unwrap();
                    //println!(
                    //    "RX Datas from input_callback : {:?}",
                    //    midi_datas_locked.get_rx_data()
                    //);
                },
                (rx, tx, ext_signal, 0),
            );

            //let mut count: i32 = 0;
            //
            //while !(midi_struct.lock().unwrap().get_signal()) {
            //    println!("In loop !");
            //    std::thread::sleep(Duration::from_secs(5));
            //    midi_struct.lock().unwrap().toggle_close_thread();
            //    count += 1;
            //}

            let stop_signal_arc = Arc::clone(int_signal);

            loop {
                if *stop_signal_arc.lock().unwrap() {
                    return;
                }
                sleep(Duration::from_millis(100));
            }
        }
        Err(e) => error!("{}", e),
    };
}

fn input_callback(
    stamp: &u64,
    mesg: &[u8],
    cc_flag: &mut CCflag,
    pass_trough: Option<&mut MidiOutputConnection>,
    //midi_struct: &mut Arc<Mutex<MiBlRustProcess>>,
    //_tx: Arc<Mutex<Sender<u8>>>,
    //rx: &mut Arc<Mutex<Sender<u8>>>,
    tx: &Sender<u64>,
    rx: &Sender<u64>,
) {
    rx.send(*stamp).unwrap();
    tx.send(*stamp).unwrap();
    //let raw_midi = RawMidi::new(*stamp, mesg).unwrap();

    //let midi_result = process_midi_mesg(stamp, &raw_midi, "MC", cc_flag);

    // Update the rx field
    //let mut midi_struct_locked = midi_struct.lock().unwrap();
    //midi_struct_locked.set_rx(*stamp, mesg);
    //
    //match midi_result {
    //    Ok(mesg) => match mesg.to_send {
    //        Some(mesg) => {
    //            info!("Connection out found! Midi mesg : {:04X?}", mesg.data());
    //
    //            match pass_trough.unwrap().send(mesg.data()) {
    //                Ok(_) => (),
    //                Err(err) => {
    //                    error!("Error when sending midi mesg to output : {}", err)
    //                }
    //            }
    //        }
    //        None => info!("Nothing to send, skip it…"),
    //    },
    //    Err(err) => println!("No midi mesg output : {}", err),
    //};
}
