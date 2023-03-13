use rainout::{MidiControlScheme, ProcessInfo, ProcessHandler, StreamInfo, Backend, RainoutConfig, RunOptions, AutoOption, MidiPortConfig, MidiConfig, AudioDeviceConfig};
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
        name: String::from("a2j:Midi Through [14] (capture): [0] Midi Through Port-0"),
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

    std::thread::sleep(std::time::Duration::from_secs(20));

    let _ = stream_handle;
}


pub struct MidiProcessor {
    mesg: String,
    test: bool,
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

        // println!("{:?}",proc_info.midi_inputs[0].events());

    }

}
