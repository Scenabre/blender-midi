use log::info;
use midir::{MidiOutputConnection, MidiOutputPort};
//use rainout::{ProcessInfo, ProcessHandler, StreamInfo, RawMidi};
//use midir::{Ignore, MidiInput};
use pyo3::prelude::*;
use simple_logger::SimpleLogger;

use std::time::Instant;

use crate::container::RawMidi;
use crate::midi_process_mesg::{process_midi_mesg, CCflag};
use crate::midi_send_mesg::initialize_mc_device;
use crate::setup_client_params::{setup_client_params, AudioParams};

#[pyfunction]
pub fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    match setup_client_params() {
        Ok(mut params) => {
            let mut conn_out: Option<MidiOutputConnection> = match params
                .midi_output
                .connect(&params.midi_output_port, "bl-midi-out")
            {
                Ok(out) => Some(out),
                Err(err) => {
                    log::error!("Unable to connect to output : {}", err);
                    None
                }
            };

            let init_mesgs: Vec<RawMidi> = initialize_mc_device().unwrap();
            log::info!("Sending all messages to midi device now…");

            let init_mesgs_len = init_mesgs.len();

            //let _ = conn_out.as_mut().unwrap().send(init_mesgs[11].data());

            for (idx, mesg) in init_mesgs.iter().enumerate() {
                log::info!(
                    "Sending mesg {}/{} : {:04X?}",
                    (idx + 1),
                    init_mesgs_len,
                    mesg.data()
                );
                let _ = conn_out.as_mut().unwrap().send(mesg.data());
            }

            log::info!("Initilization done !");

            let _conn_in = params.midi_input.connect(
                &params.midi_input_port,
                "bl-midi-in",
                move |stamp, message, _| {
                    input_callback(&stamp, message, &mut params.cc_flag, conn_out.as_mut());
                    //conn_out.as_mut()
                },
                (),
            );

            std::thread::sleep(std::time::Duration::from_secs(200));
        }
        Err(e) => log::error!("{}", e),
    };
}

fn input_callback(
    stamp: &u64,
    mesg: &[u8],
    cc_flag: &mut CCflag,
    pass_trough: Option<&mut MidiOutputConnection>,
) {
    //let start = Instant::now();

    let raw_midi = RawMidi::new(*stamp, mesg).unwrap();

    let midi_result = process_midi_mesg(stamp, &raw_midi, "MC", cc_flag);

    match midi_result {
        Ok(mesg) => {
            match mesg.to_send {
                Some(mesg) => {
                    log::info!("Connection out found ! Midi mesg : {:04X?}", mesg.data());
                    match pass_trough.unwrap().send(mesg.data()) {
                        Ok(_) => (),
                        Err(err) => {
                            log::error!("Error when sending midi mesg to output : {}", err)
                        }
                    }
                }
                None => log::info!("Nothing to send, skip it…"),
            }

            //for midi_mesg in mesg.to_send.iter() {
            //    proc_info.midi_outputs[0]
            //        .push(*midi_mesg)
            //        .unwrap_or_else(|err| {
            //            log::error!("Unable to push Midi to device : {}", err);
            //        });
            //}
        }
        Err(err) => println!("No midi mesg output : {}", err),
    };
    //let duration = start.elapsed();
    //log::info!(
    //    "[{}] Process time duration : {:?} | Time since program begin : {:?}",
    //    self.loop_num,
    //    duration,
    //    self.time.elapsed()
    //);

    //self.loop_num += 1;

    //proc_info.midi_outputs[0]
    //    .extend_from_slice(&proc_info.midi_inputs[0].events())
    //    .unwrap();
}

//pub trait ProcessHandler: 'static + Send {
//    /// Initialize/allocate any buffers here. This will only be called once on
//    /// creation.
//    fn init(&mut self, stream_info: &StreamInfo);
//
//    /// This gets called if the user made a change to the configuration that does not
//    /// require restarting the audio thread.
//    fn stream_changed(&mut self, stream_info: &StreamInfo);
//
//    /// Process the current buffers. This will always be called on a realtime thread.
//    fn process<'a>(&mut self, proc_info: ProcessInfo<'a>);
//}

//pub struct MidiProcessor {
//    debug: bool,
//    to_send: Vec<RawMidi>,
//    loop_num: u32,
//    time: Instant,
//}
//
//impl ProcessHandler for MidiProcessor {
//    fn init(&mut self, stream_info: &StreamInfo) {
//        println!("Midi processor : {}", self.debug);
//        let init_mc_dev = initialize_mc_device();
//        match init_mc_dev {
//            Ok(mesg) => {
//                self.to_send.extend(mesg);
//            }
//            Err(err) => println!("{}", err),
//        };
//        dbg!(stream_info);
//    }
//
//    fn stream_changed(&mut self, stream_info: &StreamInfo) {
//        println!("Stream info changes");
//        dbg!(stream_info);
//    }
//
//    fn process<'a>(&mut self, proc_info: ProcessInfo<'a>) {
//        proc_info.midi_outputs[0].clear();
//
//        let start = Instant::now();
//
//        if !self.to_send.is_empty() {
//            match self.to_send.pop() {
//                Some(midi_mesg) => {
//                    proc_info.midi_outputs[0]
//                        .push(midi_mesg)
//                        .unwrap_or_else(|err| {
//                            log::error!("Unable to push initialize data to midi device : {}", err);
//                        })
//                }
//                None => log::error!("Unable to get midi mesg from to_send buffer"),
//            };
//            //log::info!("Buffer : {:?}",proc_info.midi_outputs[0]);
//        };
//
//        if proc_info.midi_inputs.len() == 0 {
//            return;
//        };
//
//        let events = proc_info.midi_inputs[0].events();
//        //let mut midi_buffer;
//
//        if events.len() != 0 {
//            let midi_result = process_midi_mesg(&proc_info, events, "MC");
//
//            match midi_result {
//                Ok(mesg) => {
//                    if !mesg.to_send.is_empty() {
//                        println!("Midi mesg : {:?}", mesg.to_send);
//                        for midi_mesg in mesg.to_send.iter() {
//                            proc_info.midi_outputs[0]
//                                .push(*midi_mesg)
//                                .unwrap_or_else(|err| {
//                                    log::error!("Unable to push Midi to device : {}", err);
//                                });
//                        }
//                    }
//                }
//                Err(err) => println!("No midi mesg output : {}", err),
//            };
//            let duration = start.elapsed();
//            log::info!(
//                "[{}] Process time duration : {:?} | Time since program begin : {:?}",
//                self.loop_num,
//                duration,
//                self.time.elapsed()
//            );
//        };
//
//        self.loop_num += 1;
//
//        proc_info.midi_outputs[0]
//            .extend_from_slice(&proc_info.midi_inputs[0].events())
//            .unwrap();
//    }
//}
