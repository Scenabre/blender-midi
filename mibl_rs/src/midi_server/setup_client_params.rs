use midir::{Ignore, MidiInput, MidiInputPort, MidiOutput, MidiOutputPort};

use thiserror::Error;

use crate::midi_server::container::SIGflag;
use crate::midi_server::midi_process_mesg::CCflag;

pub struct AudioParams {
    pub port_name: String,
    pub midi_input: MidiInput,
    pub midi_input_port: MidiInputPort,
    pub midi_output: MidiOutput,
    pub midi_output_port: MidiOutputPort,
    pub cc_flag: CCflag,
    pub signal_flags: SIGflag,
}

#[derive(Error, Debug)]
pub enum ParamsInitError {
    //#[error("No midi device found")]
    //MidiDeviceNotFound,
    #[error("No input port found")]
    InputPortNotfound,
    #[error("Unable to create MidiInput")]
    MidiInputError,
    //#[error("No output port found")]
    //OutputPortNotfound,
    #[error("Unable to create MidiOutput")]
    MidiOutputError,
}

const CLIENT_NAME_IN: &str = "Blender midi - in";
const CLIENT_NAME_OUT: &str = "Blender midi - out";
const DEFAULT_PORT_NAME: &str = "UMC204HD";

pub type SetupResult = Result<AudioParams, ParamsInitError>;

pub fn setup_client_params() -> SetupResult {
    //let audio_in_ports: Vec<String> = vec![];

    let mut midi_in = match MidiInput::new(CLIENT_NAME_IN) {
        Ok(midi_in) => midi_in,
        Err(_) => return Err(ParamsInitError::MidiInputError),
    };

    let midi_out = match MidiOutput::new(CLIENT_NAME_OUT) {
        Ok(midi_out) => midi_out,
        Err(_) => return Err(ParamsInitError::MidiOutputError),
    };

    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let in_port: &MidiInputPort = match in_ports.len() {
        0 => return Err(ParamsInitError::InputPortNotfound),
        _ => {
            let mut idx_found: usize = 0;
            for (i, p) in in_ports.iter().enumerate() {
                let port_name = midi_in.port_name(p).unwrap();

                if port_name.contains(DEFAULT_PORT_NAME) {
                    idx_found = i;
                    break;
                };
            }
            in_ports
                .get(idx_found)
                .ok_or("invalid input port selected")
                .unwrap()
        }
    };

    let out_ports = midi_out.ports();

    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err(ParamsInitError::InputPortNotfound),
        _ => {
            let mut idx_found: usize = 0;
            for (i, p) in out_ports.iter().enumerate() {
                let port_name = midi_out.port_name(p).unwrap();

                if port_name.contains(DEFAULT_PORT_NAME) {
                    idx_found = i;
                    break;
                };
            }
            out_ports
                .get(idx_found)
                .ok_or("invalid input port selected")
                .unwrap()
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port).unwrap();

    println!("Connection open, reading input from '{}'…", in_port_name);

    let parameters = AudioParams {
        port_name: in_port_name,
        midi_input: midi_in,
        midi_input_port: in_port.clone(),
        midi_output: midi_out,
        midi_output_port: out_port.clone(),
        cc_flag: CCflag::default(),
        signal_flags: SIGflag::default(),
    };

    Ok(parameters)
}
