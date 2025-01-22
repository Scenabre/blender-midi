use midir::MidiOutputConnection;
use simple_logger::SimpleLogger;

use crate::container::RawMidi;
use crate::midi_process_mesg::{process_midi_mesg, CCflag};
use crate::midi_send_mesg::initialize_mc_device;
use crate::setup_client_params::setup_client_params;

pub fn init_midi_audio() {
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
    let raw_midi = RawMidi::new(*stamp, mesg).unwrap();

    let midi_result = process_midi_mesg(stamp, &raw_midi, "MC", cc_flag);

    match midi_result {
        Ok(mesg) => match mesg.to_send {
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
        },
        Err(err) => println!("No midi mesg output : {}", err),
    };
}
