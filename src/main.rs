use rainout::{MidiControlScheme, ProcessInfo, ProcessHandler, StreamInfo, Backend, RainoutConfig, RunOptions, AutoOption, MidiPortConfig, MidiConfig, RawMidi, AudioDeviceConfig};
use std::string::String;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();

    if let Ok(dev) = rainout::enumerate_midi_backend(Backend::Jack) {
        log::debug!("Midi Devices : {:?}",dev.in_ports);
    } else {
        log::error!("Unable to fetch midi devices !");
    };


    let audio_device_id = rainout::DeviceID {
        name: String::from("HDA Intel PCH, ALC3246 Analog"),
        identifier: Some(String::from("hw:0,0")),
    };

    let midi_device_id = rainout::DeviceID {
        name: String::from("jack-keyboard:midi_out"),
        // name: String::from("a2j:Midi Through [14] (capture): [0] Midi Through Port-0"),
        identifier: None,
    };

    let audio_device_conf = AudioDeviceConfig::Single(audio_device_id);

    // let vec_in_ports: Vec<String> = vec![String::from("system:capture_1"),String::from("system:capture_2")];
    let audio_in_ports: Vec<String> = vec![];
    // let vec_out_ports: Vec<String> = vec![String::from("system:playback_1"), String::from("system:playback_2")];
    let audio_out_ports: Vec<String> = vec![];

    let jack_ports = AudioDeviceConfig::Jack {
        in_ports: audio_in_ports,
        out_ports: audio_out_ports,
    };

    let midi_in_ports: Vec<MidiPortConfig> = vec![ MidiPortConfig {
        device_id: midi_device_id,
        port_index: 0,
        control_scheme: MidiControlScheme::default(),
    }];

    let midi_out_ports: Vec<MidiPortConfig> = vec![];

    let midi_conf = MidiConfig {
        midi_backend: AutoOption::Use(Backend::Jack),
        in_ports: AutoOption::Use(midi_in_ports),
        out_ports: AutoOption::Use(midi_out_ports),
    };

    let config = RainoutConfig {
        audio_backend: AutoOption::Use(Backend::Jack),
        audio_device: audio_device_conf,
        sample_rate: AutoOption::Use(48000),
        block_size: AutoOption::Use(512),
        take_exclusive_access: false,
        midi_config: Some(midi_conf),
    };

    let run_opt = RunOptions {
        use_application_name: Some(String::from("Blender Midi Rust")),
        auto_audio_inputs: false,
        midi_buffer_size: 1024,
        check_for_silent_inputs: false,
        must_have_stereo_output: false,
        empty_buffers_for_failed_ports: false,
        max_buffer_size: 1024,
        msg_buffer_size: 512,
    };

    let process = MidiProcessor { mesg: String::from("Midi processor!"), test: true};

    let stream_handle = rainout::run(&config,&run_opt,process).unwrap(); 

    std::thread::sleep(std::time::Duration::from_secs(200));

    let _ = stream_handle;
}


pub struct MidiProcessor {
    mesg: String,
    test: bool,
}

// pub struct MidiMesg {
//     cmd: u8,
//     param1: u8,
//     param2: Option<u8>,
// }

// pub enum MidiMesg {
//     Cmd,
//     Param1,
//     Param2,
// }

pub fn process_midi_mesg(event: &[u8]) -> u8 {

// Command  Meaning      # parameters  param 1      param 2
// 0x80      Note-off    2              key          velocity
// 0x90      Note-on     2              key          velocity
// 0xA0      Aftertouch  2              key          touch
// 0xB0      Cont CTRL   2              ctrl #       ctrl value
// 0xC0      Patch chg   2              instr # 	
// 0xD0      Chan Press  1              pressure     X
// 0xE0      Pitch bend  2              lsb (7 bits) msb (7 bits)
// 0xF0      (non-musical commands) 			
    
    let cmd: u8 = event[0];

    match cmd {
        0xA0|0xC0|0xE0 => println!("Command not used in Blender Midi"),
        0x80 => println!("Note off : {:?} (vel : {:?})", event[1], event[2]),
        0x90 => println!("Note on : {:?} (vel : {:?})", event[1], event[2]),
        0xD0 => println!("Channel press : {:?}", event[1]),
        0xFF => panic!("User press PANIC on keyboard, shutdown client !"),
        _ => println!("Unkown event : {:04X?}", event),
    }

    return 0;
}

impl ProcessHandler for MidiProcessor {
    fn init(&mut self, stream_info: &StreamInfo) {
        dbg!(stream_info);
        if self.test {
            println!("{:?}",self.mesg);
        }
    }

    fn stream_changed(&mut self, stream_info: &StreamInfo) {
        println!("Stream info changes");
        dbg!(stream_info);
    }

    fn process<'a>(&mut self, proc_info: ProcessInfo<'a>) {

        let events = proc_info.midi_inputs[0].events();

        if events.len() != 0 {
            if events.len() > 1 {
                for event in events.iter() {
                    process_midi_mesg(event.data());
                }
            } else {
                let event: RawMidi = events[0];
                process_midi_mesg(event.data());
            };
        };
    }

}


