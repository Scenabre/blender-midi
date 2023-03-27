use rainout::{ProcessInfo, ProcessHandler, StreamInfo};
//use std::string::String;
use simple_logger::SimpleLogger;
//use clap::{arg, command, value_parser, ArgAction, Command};

use setup_client_params::setup_client_params;
use midi_process_mesg::process_midi_mesg;

mod setup_client_params;
mod midi_process_mesg;

fn main() {
    //let args = ""; 

    SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();

    match setup_client_params() {
        Ok(params) => {
            let process = MidiProcessor {
                debug: false,
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
}

impl ProcessHandler for MidiProcessor {

    fn init(&mut self, stream_info: &StreamInfo) {
        println!("Midi processor : {}", self.debug);
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

            let midi_result = process_midi_mesg(events,"MC");

            match midi_result {
                Ok(mesg) => println!("Midi mesg : {:?}", mesg),
                Err(err) => println!("No midi mesg output : {}", err),
            };
        };

        proc_info.midi_outputs[0].clear_and_copy_from(&proc_info.midi_inputs[0]);
    }

}
