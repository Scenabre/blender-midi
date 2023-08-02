use rainout::{RawMidi, ProcessInfo};
use crate::midi_event::trigger_midi_events;

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
    pub result: [Vec<MidiMesg>;16],
    pub to_send: Vec<RawMidi>,
}

pub type MidiResult = Result<MidiProcess, &'static str>;

const CHROM_RANGE: [&str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

pub fn process_midi_mesg(proc_info: &ProcessInfo, events: &[RawMidi], protocole: &str) -> MidiResult {

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
    const INIT: Vec<MidiMesg> = Vec::new();

    let mut mesg_send: [Vec<MidiMesg>;SIZE] = [INIT;SIZE];
    let mut midi_mesg: MidiMesg = MidiMesg::new();
    let mut midi_to_send: Vec<RawMidi> = vec![];

    let proto = match protocole {
        "HUI" => 0,
        "MC"|"Mackie Control"|"MackieControl" => 1,
        _ => {
            log::warn!("Protocole unknown drop to HUI");
            0
        }
    };

    fn get_note_name(note: u8) -> &'static str {

        match note {
            21 ..= 127 => {
                let note_idx: usize = ((note-21)%12).into();
                return CHROM_RANGE[note_idx]; 
            },
            0 ..= 20 => return "NiR",
            _ => log::error!("Note value superior to 127 !"),
        }

        ""
    }

    fn get_octave(note: u8) -> u8 {

        match note {
            21 ..= 127 => {
                return (note/12)-1;
            },
            _ => log::warn!("Note not in standard range !"),
        }

        10
    }

    fn get_channel(cmd: u8) -> u8 {
        return ((cmd << 4) >> 4)+1;
    }

    fn convert_half(vel: u8) -> f32 {
        return vel as f32 / 127.0;
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

        return midi_mesg;
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

        let _cc_msb_range: Vec<u8> = vec![0,1,2,4,5,6,7,8,10,11,12,13,16,17,18,19,98,100];
        let _cc_lsb_range: Vec<u8> = vec![32,33,34,36,37,38,39,40,42,43,44,45,48,49,50,51,99,101];

        let mut cc_high: Vec<u8> = (64..85).collect();
        let mut cc_range_2: Vec<u8> = (91..98).collect();
        cc_high.push(88);
        cc_high.append(&mut cc_range_2);

        fn print_cc_value(cc_num: u8, cc_value: u16) {

            match cc_num {
                0x00 => println!("CC not used in Blender Midi"),
                0x01 => println!("Modulation wheel : {}", cc_value),
                0x05 => println!("Portamento : {}", cc_value),
                _ =>  println!("CC #{} : {}", cc_num, cc_value),
            }
        }

        match cc_lsb_value {
            None => {
                midi_mesg.value = convert_half(cc_msb_value);
                print_cc_value(cc_num, cc_msb_value as u16);
            },
            Some(cc_lsb_value) => {
                let u16_cc_value = (cc_msb_value as u16) << 7 | cc_lsb_value as u16;
                midi_mesg.value = (u16_cc_value as f32)/16384.0;
                print_cc_value(cc_num, u16_cc_value);
            },
        };

        midi_mesg.name = format!("CC #{}", cc_num);

        return midi_mesg;
    }

    fn process_sys(event : &[u8]) {

        println!("SysEx not implemented yet !");

        if ! event.contains(&0xF7) {
            log::warn!("SysEx send without tail !");
        }

        let _end_pos = event.iter().position(|&x| x == 0xF7);
    }

    fn process_pitch_bend(pitch: (u8,u8)) -> MidiMesg {

        let mut midi_mesg = MidiMesg::new();

        let (lsb,msb) = pitch;
        let u16_pitch = (msb as u16) << 7 | lsb as u16;
        let norm_pitch = (u16_pitch as f32)/16384.0;
        println!("Pitch bend values : {}", norm_pitch);

        midi_mesg.name = "Pitch bend".to_string();
        midi_mesg.value = norm_pitch;

        return midi_mesg;
    }

    let mut cc_lsb_flag = false;
    let mut cc_msb_val_save: u8 = 0;
    let mut cc_num_save: u8 = 0;

    let mut display_events: Vec<&[u8]> = vec![];

    for event in events.iter() {
        display_events.push(event.data());
    }

    println!("\n ---------\n  Midi event to process ({}:{}) : {:04X?}\n ---------\n", proto, protocole, display_events);

    for (idx, event) in events.iter().enumerate() {
        let event = event.data();

        let cmd = event[0];

        if cmd == 0xFF {
            return Err("User press PANIC on midi device !");
        }

        match trigger_midi_events(&proc_info, event) {
            Ok(raw_midi) => midi_to_send.extend(raw_midi),
            Err(err) => log::warn!("Trigger event dropped in process mesg ! Debug : {}", err),
        }

        midi_to_send.extend(trigger_midi_events(&proc_info, event).unwrap());

        let channel: u8 = get_channel(cmd);
        let chan_idx: usize = usize::from(channel-1);

        println!("CHANNEL : {}", channel);

        let clean_cmd = (cmd >> 4) << 4;

        println!("Raw MIDI : {:04X?} {:?}", event, event);

        match clean_cmd {
            0x80|0x90 => {
                midi_mesg = process_note(clean_cmd, event[1], event[2]);
                println!("{}",midi_mesg.name);
            }
            0xA0 => {
                let poly_key_value = convert_half(event[2]);
                midi_mesg = MidiMesg {
                    channel,
                    name : format!("Poly Key Pressure Aftertouch on {}{}", get_note_name(event[1]), get_octave(event[1])),
                    value : poly_key_value,
                };
                println!("{} : {}", midi_mesg.name, midi_mesg.value);
            }
            0xB0 => {
                match event[1] {
                    cc_num if cc_num > 0x3F && cc_num < 0x62 => {
                        midi_mesg = process_cc(cc_num, event[2], None);
                    },
                    cc_num if cc_num <= 0x1F && cc_lsb_flag == false => {

                        let last_idx = events.len() - 1;

                        if idx != last_idx {
                            let next_clean_cmd = (events[idx+1].data()[0] >> 4) << 4;
                            let next_channel = get_channel(events[idx+1].data()[0]);

                            if next_channel == channel && next_clean_cmd == clean_cmd {
                                if events[idx+1].data()[1] == event[1]+0x20 {
                                    cc_lsb_flag = true;
                                    cc_msb_val_save = event[2];
                                    cc_num_save = cc_num;
                                }
                            }
                        }

                        if cc_lsb_flag == false {
                            midi_mesg = process_cc(cc_num, event[2], None);
                        }
                    },
                    cc_num if cc_num > 0x1F && cc_num < 0x40 => {
                        if cc_lsb_flag == false {
                            midi_mesg = process_cc(cc_num, event[2], None);
                        } else {
                            midi_mesg = process_cc(cc_num_save, cc_msb_val_save, Some(event[2]));
                            cc_lsb_flag = false;
                        }
                    },
                    _ => log::warn!("Unknown value found for CC !"),
                }
            },
            0xC0 => println!("Command not used in Blender Midi : {:04X?}", cmd),
            0xD0 => {
                let chan_press_value = convert_half(event[1]);

                println!("Channel Pressure Aftertouch : {}", chan_press_value);

                midi_mesg = MidiMesg {
                    channel,
                    name : "Channel Pressure Aftertouch".to_string(),
                    value : chan_press_value,
                };
            }
            0xE0 => {
                midi_mesg = process_pitch_bend((event[1],event[2]));
            },
            0xF0 => process_sys(event),
            _ => log::warn!("Unkown event : {:04X?}", event),
        }

        if ! midi_mesg.name.is_empty() {
            mesg_send[chan_idx].push(midi_mesg.clone());
        }

        println!("----");
    }

    let midi_to_send: MidiProcess = MidiProcess {
        result: mesg_send,
        to_send: midi_to_send, 
    };

    Ok(midi_to_send)
}
