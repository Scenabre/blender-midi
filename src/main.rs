use rainout::{ProcessInfo, ProcessHandler, StreamInfo, RawMidi};
//use std::string::String;
use simple_logger::SimpleLogger;
//use clap::{arg, command, value_parser, ArgAction, Command};
use std::time::{Instant};

use setup_client_params::setup_client_params;
use midi_process_mesg::process_midi_mesg;
use midi_send_mesg::initialize_mc_device;

mod setup_client_params;
mod midi_process_mesg;
mod midi_event;
mod midi_send_mesg;

fn main() {
    //let args = ""; 

    SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();

    match setup_client_params() {
        Ok(params) => {
            let process = MidiProcessor {
                debug: false,
                to_send: Vec::new(),
                loop_num: 0,
                time: Instant::now(),
            };

            let stream_handle = rainout::run(&params.config,&params.run_opt,process).unwrap(); 

            std::thread::sleep(std::time::Duration::from_secs(200));

            let _ = stream_handle;

        },
        Err(e) => log::error!("{}",e),
    };
   
   }

pub struct MidiProcessor {
    debug : bool,
    to_send : Vec<RawMidi>,
    loop_num: u32,
    time: Instant,
}

impl ProcessHandler for MidiProcessor {

    fn init(&mut self, stream_info: &StreamInfo) {
        println!("Midi processor : {}", self.debug);
        let init_mc_dev = initialize_mc_device();
        match init_mc_dev {
            Ok(mesg) => {
                self.to_send.extend(mesg);
            },
            Err(err) => println!("{}",err),
        };
        dbg!(stream_info);
    }

    fn stream_changed(&mut self, stream_info: &StreamInfo) {
        println!("Stream info changes");
        dbg!(stream_info);
    }

    fn process<'a>(&mut self, proc_info: ProcessInfo<'a>) {

        proc_info.midi_outputs[0].clear();

        let start = Instant::now();

        if ! self.to_send.is_empty() {
            match self.to_send.pop() {
                Some(midi_mesg) => proc_info.midi_outputs[0].push(midi_mesg).unwrap_or_else(|err| {
                    log::error!("Unable to push initialize data to midi device : {}",err);
                }),
                None => log::error!("Unable to get midi mesg from to_send buffer")

            };
            log::info!("Buffer : {:?}",proc_info.midi_outputs[0]);
        };

        if proc_info.midi_inputs.len() == 0 {
            return;
        };

        let events = proc_info.midi_inputs[0].events();
        //let mut midi_buffer;

        if events.len() != 0 {

            let midi_result = process_midi_mesg(&proc_info, events,"MC");

            match midi_result {
                Ok(mesg) => { 
                    if ! mesg.to_send.is_empty() {
                        println!("Midi mesg : {:?}", mesg.to_send);
                        for midi_mesg in mesg.to_send.iter() {
                            proc_info.midi_outputs[0].push(*midi_mesg).unwrap_or_else(|err| {
                                log::error!("Unable to push Midi to device : {}",err);
                            });
                        }
                    }
                },
                Err(err) => println!("No midi mesg output : {}", err),
            };
            let duration = start.elapsed();
            log::info!("[{}] Process time duration : {:?} | Time since program begin : {:?}",self.loop_num, duration, self.time.elapsed());
        };

        self.loop_num += 1;

        proc_info.midi_outputs[0].extend_from_slice(&proc_info.midi_inputs[0].events()).unwrap(); 
    }

}
