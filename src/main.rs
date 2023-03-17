use rainout::{MidiControlScheme, ProcessInfo, ProcessHandler, StreamInfo, Backend, RainoutConfig, RunOptions, AutoOption, MidiPortConfig, MidiConfig, RawMidi, AudioDeviceConfig};
use std::string::String;
use simple_logger::SimpleLogger;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let args = ""; 



    SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();
   
    let audio_in_ports: Vec<String> = vec![];
    let audio_out_ports: Vec<String> = vec![];

    let audio_device_conf = AudioDeviceConfig::Jack {
        in_ports: audio_in_ports,
        out_ports: audio_out_ports,
    };

    let midi_devices = rainout::enumerate_midi_backend(Backend::Jack).unwrap();

    if midi_devices.in_ports.len() == 0 || midi_devices.out_ports.len() == 0 {
        log::error!("No midi device found !");
    }

    let default_midi_in_dev = midi_devices.in_ports[1].id.clone();
    let default_midi_out_dev = midi_devices.out_ports[0].id.clone();

    let midi_in_ports: Vec<MidiPortConfig> = vec![ MidiPortConfig {
        device_id: default_midi_in_dev,
        port_index: 0,
        control_scheme: MidiControlScheme::default(),
    }];

    let midi_out_ports: Vec<MidiPortConfig> = vec![ MidiPortConfig {
        device_id: default_midi_out_dev,
        port_index: 0,
        control_scheme: MidiControlScheme::default(),
    }];

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

const CHROM_RANGE: [&str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

pub type MidiResult = Result<u8, &'static str>;

pub fn process_midi_mesg(event: &[u8]) -> MidiResult {

// CHANNEL VOICE MESG
// Command  Meaning      # parameters  param 1      param 2
// 0x80      Note-off    2              key          velocity
// 0x90      Note-on     2              key          velocity
// 0xA0      Aftertouch  2              key          touch
// 0xB0      Cont CTRL   2              ctrl #       ctrl value (0-119)
// 0xC0      Prog chg    2              instr # 	
// 0xD0      Chan Press  1              pressure     X
// 0xE0      Pitch bend  2              lsb (7 bits) msb (7 bits)
// 0xF0      (non-musical commands) 			
    
    let cmd = event[0];

    if cmd == 0xFF {
        return Err("User press PANIC on midi device, shutdown client !");
    }

    fn get_note_name(note: u8) -> &'static str {
        if note < 21 || note > 108 {
            log::warn!("Note not in chromatic range !");
            return "";
        }

        let note_idx: usize = ((note-21)%12).into();

       return CHROM_RANGE[note_idx]; 
    }

    fn get_octave(note: u8) -> u8 {
        if note < 21 || note > 108 {
            log::warn!("Note not in chromatic range !");
            return 0;
        }
        return (note/12)-1;

    }

    fn get_channel(cmd: u8) -> u8 {
        return ((cmd << 4) >> 4)+1;
    }

    fn convert_half(vel: u8) -> f32 {
        return vel as f32 / 127.0;
    }

    fn process_note(cmd: u8, note: u8, vel: u8) {
        if cmd == 0x80 || vel == 0 {
            println!("Note off : {}{}", get_note_name(note),get_octave(note));
            return;
        }

        println!("Note on : {}{} (vel: {})",get_note_name(note),get_octave(note),convert_half(vel));

    }

    fn process_cc(cc_type: u8, cc_value: u8) {
        if cc_type < 0x1F {
            match cc_type {
                0x01 => println!("Modulation wheel : {}", convert_half(cc_value)),
                0x05 => println!("Portamento : {}", convert_half(cc_value)),
                _ =>  println!("CC #{} : {}", cc_type, cc_value),
            }
        }
    }

    fn process_pitch_bend(pitch: u8) {
        let norm_pitch: f32 = (pitch as f32 - 64.0) / 64.0;
        println!("Pitch bend value : {}",norm_pitch); 
    }

    println!("CHANNEL : {}", get_channel(cmd));

    let clean_cmd = (cmd >> 4) << 4;

    println!("Raw MIDI : {:04X?}", event);

    match clean_cmd {
        0xA0|0xC0 => println!("Command not used in Blender Midi : {:04X?}", cmd),
        0xB0 => process_cc(event[1],event[2]),
        0x80|0x90 => process_note(clean_cmd, event[1], event[2]),
        0xD0 => println!("Channel press : {}", event[1]),
        0xE0 => process_pitch_bend(event[2]),
        _ => log::warn!("Unkown event : {:04X?}", event),
    }

    Ok(0)
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

        if proc_info.midi_inputs.len() == 0 {
            return;
        };

        let events = proc_info.midi_inputs[0].events();

        if events.len() != 0 {
            if events.len() > 1 {
                for event in events.iter() {
                    if let Err(e) = process_midi_mesg(event.data()) {
                        panic!("{}",e);
                    }
                }
            } else {
                let event: RawMidi = events[0];
                if let Err(e) = process_midi_mesg(event.data()) {
                    panic!("{}",e);
                }
            };
        }; 

        //proc_info.midi_outputs[0].clear_and_copy_from(&proc_info.midi_inputs[0]);
    }

}


