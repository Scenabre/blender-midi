use rainout::{ProcessInfo, ProcessHandler, StreamInfo, RawMidi};
use std::string::String;
use simple_logger::SimpleLogger;
use clap::{arg, command, value_parser, ArgAction, Command};

use setup_client_params::setup_client_params;
use midi_process_mesg::process_midi_mesg;

mod setup_client_params;
mod midi_process_mesg;

fn main() {
    let args = ""; 

    SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();

    match setup_client_params() {
        Ok(params) => {
            let process = MidiProcessor {
                debug: true,
            };

            let stream_handle = rainout::run(&params.config,&params.run_opt,process).unwrap(); 

            std::thread::sleep(std::time::Duration::from_secs(200));

            let _ = stream_handle;

        },
        Err(e) => log::error!("{}",e),
    };
   
   }

pub struct MidiProcessor {
    debug: bool,
}

impl ProcessHandler for MidiProcessor {

    fn init(&mut self, stream_info: &StreamInfo) {
        dbg!(stream_info);
    }

    fn stream_changed(&mut self, stream_info: &StreamInfo) {
        println!("Stream info changes");
        dbg!(stream_info);
    }

    fn process<'a>(&mut self, proc_info: ProcessInfo<'a>) {

        if proc_info.midi_inputs.len() == 0 {
            return;
        };

        let events = proc_info.midi_inputs[0].events();

        if events.len() != 0 {

            process_midi_mesg(events);
            // if events.len() > 1 {
            //     for event in events.iter() {
            //         if let Err(e) = process_midi_mesg(event.data(), cc_flag) {
            //             panic!("{}",e);
            //         }
            //     }
            // } else {
            //     let event: RawMidi = events[0];
            //     if let Err(e) = process_midi_mesg(event.data(), cc_flag) {
            //         panic!("{}",e);
            //     }
            // };
        }; 

        //proc_info.midi_outputs[0].clear_and_copy_from(&proc_info.midi_inputs[0]);
    }

}
