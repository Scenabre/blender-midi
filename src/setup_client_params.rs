use rainout::{MidiControlScheme, Backend, RainoutConfig, RunOptions, AutoOption, MidiPortConfig, MidiConfig, AudioDeviceConfig};
use thiserror::Error;

pub struct AudioParams {
    pub config: RainoutConfig,
    pub run_opt: RunOptions,
}

#[derive(Error, Debug)]
pub enum ParamsInitError {
    #[error("No midi device found")]
    MidiDeviceNotFound,
}

pub type SetupResult = Result<AudioParams, ParamsInitError>;

pub fn setup_client_params() -> SetupResult {

    let audio_in_ports: Vec<String> = vec![];
    let audio_out_ports: Vec<String> = vec![];

    let audio_device_conf = AudioDeviceConfig::Jack {
        in_ports: audio_in_ports,
        out_ports: audio_out_ports,
    };

    let midi_devices = rainout::enumerate_midi_backend(Backend::Jack).unwrap();

    if midi_devices.in_ports.len() == 0 || midi_devices.out_ports.len() == 0 {
       return Err(ParamsInitError::MidiDeviceNotFound);
    }

    let default_midi_in_dev = midi_devices.in_ports[0].id.clone();
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

    let rainout_config = RainoutConfig {
        audio_backend: AutoOption::Use(Backend::Jack),
        audio_device: audio_device_conf,
        sample_rate: AutoOption::Use(48000),
        block_size: AutoOption::Use(512),
        take_exclusive_access: false,
        midi_config: Some(midi_conf),
    };

    let rainout_run_opt = RunOptions {
        use_application_name: Some(String::from("Blender Midi Rust")),
        auto_audio_inputs: false,
        midi_buffer_size: 1024,
        check_for_silent_inputs: false,
        must_have_stereo_output: false,
        empty_buffers_for_failed_ports: false,
        max_buffer_size: 1024,
        msg_buffer_size: 512,
    };

    let parameters = AudioParams {
        config: rainout_config,
        run_opt: rainout_run_opt,
    };

    Ok(parameters)
}
