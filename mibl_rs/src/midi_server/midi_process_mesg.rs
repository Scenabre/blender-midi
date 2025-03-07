use crate::midi_server::container::{Event, RawMidi};
use crate::midi_server::midi_event::trigger_midi_events;
//use rainout::{ProcessInfo, RawMidi};

#[derive(Debug, Clone)]
pub struct MidiMesg {
    pub channel: u8,
    pub name: String,
    pub value: f32,
}

impl MidiMesg {
    fn new() -> Self {
        Self {
            channel: 0,
            name: "".to_string(),
            value: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct MidiProcess {
    pub result: MidiMesg,
    pub to_send: Option<Vec<RawMidi>>,
}

pub type MidiResult = Result<MidiProcess, &'static str>;

#[derive(Debug, Clone, Copy, Default)]
pub struct CCflag {
    cc_lsb_flag: bool,
    cc_channel: u8,
    cc_num: u8,
    cc_msb_value: u8,
    cc_note: u8,
}

impl CCflag {
    pub fn new(
        cc_lsb_flag: bool,
        cc_channel: u8,
        cc_num: u8,
        cc_msb_value: u8,
        cc_note: u8,
    ) -> Self {
        Self {
            cc_lsb_flag,
            cc_channel,
            cc_num,
            cc_msb_value,
            cc_note,
        }
    }
}

//impl SIGflag {
//    pub fn new(reset_signal: bool, bang_signal: bool) -> Self {
//        Self {
//            reset_signal,
//            trigger_reset_device,
//            bang_signal,
//        }
//    }
//}

const CHROM_RANGE: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

pub fn process_midi_mesg(
    event: &RawMidi,
    protocole: &str,
    cc_flag: &mut CCflag,
    triggers: Option<&Vec<Event>>,
) -> MidiResult {
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

    const SIZE: usize = 16;

    let mut mesg_send: Vec<MidiMesg> = Vec::new();
    let mut midi_mesg: MidiMesg = MidiMesg::new();

    let proto = match protocole {
        "HUI" => 0,
        "MC" | "Mackie Control" | "MackieControl" => 1,
        _ => {
            log::warn!("Protocole unknown drop to HUI");
            0
        }
    };

    fn get_note_name(note: u8) -> &'static str {
        match note {
            21..=127 => {
                let note_idx: usize = ((note - 21) % 12).into();
                return CHROM_RANGE[note_idx];
            }
            0..=20 => return "NiR",
            _ => log::error!("Note value superior to 127 !"),
        }

        ""
    }

    fn get_octave(note: u8) -> u8 {
        match note {
            21..=127 => {
                return (note / 12) - 1;
            }
            _ => log::warn!("Note not in standard range !"),
        }

        10
    }

    fn get_channel(cmd: u8) -> u8 {
        ((cmd << 4) >> 4) + 1
    }

    fn convert_half(vel: u8) -> f32 {
        vel as f32 / 127.0
    }

    fn process_note(cmd: u8, note: u8, vel: u8) -> MidiMesg {
        let mut midi_mesg = MidiMesg::new();

        let note_name = get_note_name(note);
        let note_octave = get_octave(note);
        let mut note_num: u8 = 0;

        if note_octave == 10 {
            note_num = note;
        }

        if cmd == 0x80 || vel == 0 {
            if note_octave == 10 {
                println!("Note off : {}", note_num);
                midi_mesg.name = format!("Note off : {}", note_num);
                midi_mesg.value = 0.0;
                return midi_mesg;
            }
            println!("Note off : {}{}", note_name, note_octave);
            midi_mesg.name = format!("Note off : {}{}", note_name, note_octave);
            midi_mesg.value = 0.0;
            return midi_mesg;
        }

        let note_vel = convert_half(vel);

        if note_octave == 10 {
            println!("Note on : {} (vel: {})", note_num, note_vel);
            midi_mesg.name = format!("Note on : {} (vel: {})", note_num, note_vel);
            midi_mesg.value = note_vel;
            return midi_mesg;
        }

        println!("Note on : {}{} (vel: {})", note_name, note_octave, note_vel);
        midi_mesg.name = format!("Note on : {}{} (vel: {})", note_name, note_octave, note_vel);
        midi_mesg.value = note_vel;

        midi_mesg
    }

    fn process_cc(cc_num: u8, cc_msb_value: u8, cc_lsb_value: Option<u8>) -> MidiMesg {
        // SYSEX TEST Mackie Control
        // https://www.simon-florentin.fr/blog/2016/12/mcp-un-peu-de-theorie-avec-les-sysex/index.html
        // SYSEX = Header + Mesg + 0xF7
        // Header = F0 00 00 66 00
        // 00 00 66 (id vendor)
        // 00 (id product)
        // Mesg sur LCD = F0 00 00 66 00 12 pos mesg F7
        // pos = 00 pour la première ligne et 38 pour la seconde ligne
        // 7 char par ligne donc 0x00 jsq 0x37 et 0x38 jsq 0x6F

        let mut midi_mesg = MidiMesg::new();

        let _cc_msb_range: Vec<u8> = vec![
            0, 1, 2, 4, 5, 6, 7, 8, 10, 11, 12, 13, 16, 17, 18, 19, 98, 100,
        ];
        let _cc_lsb_range: Vec<u8> = vec![
            32, 33, 34, 36, 37, 38, 39, 40, 42, 43, 44, 45, 48, 49, 50, 51, 99, 101,
        ];

        let mut cc_high: Vec<u8> = (64..85).collect();
        let mut cc_range_2: Vec<u8> = (91..98).collect();
        cc_high.push(88);
        cc_high.append(&mut cc_range_2);

        fn print_cc_value(cc_num: u8, cc_value: u16) {
            match cc_num {
                0x00 => println!("CC not used in Blender Midi"),
                0x01 => println!("Modulation wheel : {}", cc_value),
                0x05 => println!("Portamento : {}", cc_value),
                _ => println!("CC #{} : {}", cc_num, cc_value),
            }
        }

        match cc_lsb_value {
            None => {
                midi_mesg.value = convert_half(cc_msb_value);
                print_cc_value(cc_num, cc_msb_value as u16);
            }
            Some(cc_lsb_value) => {
                let u16_cc_value = (cc_msb_value as u16) << 7 | cc_lsb_value as u16;
                midi_mesg.value = (u16_cc_value as f32) / 16384.0;
                print_cc_value(cc_num, u16_cc_value);
            }
        };

        midi_mesg.name = format!("CC #{}", cc_num);

        midi_mesg
    }

    fn process_sys(event: &[u8]) {
        println!("SysEx not implemented yet !");

        if !event.contains(&0xF7) {
            log::warn!("SysEx send without tail !");
        }

        let _end_pos = event.iter().position(|&x| x == 0xF7);
    }

    fn process_pitch_bend(pitch: (u8, u8)) -> MidiMesg {
        let mut midi_mesg = MidiMesg::new();

        let (lsb, msb) = pitch;
        let u16_pitch = (msb as u16) << 7 | lsb as u16;
        let norm_pitch = (u16_pitch as f32) / 16384.0;
        println!("Pitch bend values : {}", norm_pitch);

        midi_mesg.name = "Pitch bend".to_string();
        midi_mesg.value = norm_pitch;

        midi_mesg
    }

    let display_event = event.data();

    println!(
        "\n ---------\n  Midi event to process ({}:{}) : {:04X?}\n ---------\n",
        proto, protocole, display_event
    );

    println!("Delta frames : {:?}", event.delta_frames);

    let event_data = event.data();

    let cmd = event_data[0];

    if cmd == 0xFF {
        return Err("User press PANIC on midi device !");
    }

    let midi_mesg_to_send: Option<Vec<RawMidi>> = match triggers {
        Some(triggers) => match trigger_midi_events(event.delta_frames(), event_data, triggers) {
            Ok(raw_midi) => raw_midi,
            Err(err) => {
                println!("Error when query events vector : {}", err);
                None
            }
        },
        None => None,
    };

    let channel: u8 = get_channel(cmd);
    let chan_idx: usize = usize::from(channel - 1);

    println!("CHANNEL : {}", channel);

    let clean_cmd = (cmd >> 4) << 4;

    println!("Raw MIDI : {:04X?} {:?}", event_data, event_data);

    match clean_cmd {
        0x80 | 0x90 => {
            midi_mesg = process_note(clean_cmd, event_data[1], event_data[2]);
            println!("{}", midi_mesg.name);
        }
        0xA0 => {
            let poly_key_value = convert_half(event_data[2]);
            midi_mesg = MidiMesg {
                channel,
                name: format!(
                    "Poly Key Pressure Aftertouch on {}{}",
                    get_note_name(event_data[1]),
                    get_octave(event_data[1])
                ),
                value: poly_key_value,
            };
            println!("{} : {}", midi_mesg.name, midi_mesg.value);
        }
        0xB0 => match event_data[1] {
            cc_num if cc_num > 0x3F && cc_num < 0x62 => {
                midi_mesg = process_cc(cc_num, event_data[2], None);
            }
            cc_num if cc_num <= 0x1F && !cc_flag.cc_lsb_flag => {
                if channel == cc_flag.cc_channel && cc_flag.cc_note == event_data[1] + 0x20 {
                    cc_flag.cc_lsb_flag = true;
                    cc_flag.cc_msb_value = event_data[2];
                    cc_flag.cc_num = cc_num;
                }

                if !cc_flag.cc_lsb_flag {
                    midi_mesg = process_cc(cc_num, event_data[2], None);
                }
            }
            cc_num if cc_num > 0x1F && cc_num < 0x40 => {
                if !cc_flag.cc_lsb_flag {
                    midi_mesg = process_cc(cc_num, event_data[2], None);
                } else {
                    midi_mesg =
                        process_cc(cc_flag.cc_num, cc_flag.cc_msb_value, Some(event_data[2]));
                    cc_flag.cc_lsb_flag = false;
                }
            }
            _ => log::warn!("Unknown value found for CC !"),
        },
        0xC0 => println!("Command not used in Blender Midi : {:04X?}", cmd),
        0xD0 => {
            let chan_press_value = convert_half(event_data[1]);

            println!("Channel Pressure Aftertouch : {}", chan_press_value);

            midi_mesg = MidiMesg {
                channel,
                name: "Channel Pressure Aftertouch".to_string(),
                value: chan_press_value,
            };
        }
        0xE0 => {
            midi_mesg = process_pitch_bend((event_data[1], event_data[2]));
        }
        0xF0 => process_sys(event_data),
        _ => log::warn!("Unkown event : {:04X?}", event_data),
    }

    let midi_to_send: MidiProcess = MidiProcess {
        result: midi_mesg,
        to_send: midi_mesg_to_send,
    };

    Ok(midi_to_send)
}
