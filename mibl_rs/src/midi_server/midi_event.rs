use crate::midi_server::container::{Event, MidiMesg, Recipe};
use crate::midi_server::sys_event::SYS_EVENT_ARRAY;
use crate::node_utils::sys_event::convert_half;

const SIZE: usize = 16;
const EPSILON: f32 = 0.01;
const CHROM_RANGE: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

// CHANNEL VOICE MESG
// Command  Meaning      # parameters  param 1      param 2
// 0x80      Note-off    2              key          velocity
// 0x90      Note-on     2              key          velocity
// 0xA0      Aftertouch  2              key          touch
// 0xB0      Cont CTRL   2              ctrl #       ctrl value (0-119)
// 0xC0      Prog chg    2              instr #
// 0xD0      Chan Press  1              pressure     X
// 0xE0      Pitch bend  2              lsb (7 bits) msb (7 bits)
// 0xF0     (non-musical commands)

pub fn craft_recipe(use_sys: &bool, custom_events: Option<&Recipe>) -> Result<Vec<Event>, String> {
    let mut events: Vec<Event> = Vec::new();
    let mut event_idx: u64 = 0;

    if *use_sys {
        for (idx, (value, name)) in SYS_EVENT_ARRAY.iter().enumerate() {
            event_idx = idx as u64;
            let tmp_events = match *value {
                0x3C => {
                    if *name == "TRANS_Wheel" {
                        vec![
                            Event::new(
                                event_idx,
                                name.to_string(),
                                vec![0xB0, *value, 0x01],
                                None,
                                Some(1.0),
                                0,
                                None,
                                false,
                            ),
                            Event::new(
                                event_idx,
                                name.to_string(),
                                vec![0xB0, *value, 0x41],
                                None,
                                Some(-1.0),
                                0,
                                None,
                                false,
                            ),
                        ]
                    } else {
                        vec![Event::new(
                            event_idx,
                            name.to_string(),
                            vec![0x90, *value, 0x7F],
                            Some(vec![vec![0x90, *value, 0x00]]),
                            Some(1.0),
                            0,
                            None,
                            true,
                        )]
                    }
                }
                0x71..=0x73 => vec![Event::new(
                    event_idx,
                    name.to_string(),
                    vec![],
                    Some(vec![vec![0x90, 0, 0]]),
                    Some(1.0),
                    0,
                    None,
                    true,
                )],
                _ => vec![Event::new(
                    event_idx,
                    name.to_string(),
                    vec![0x90, *value, 0x7F],
                    Some(vec![vec![0x90, *value, 0x00]]),
                    Some(1.0),
                    0,
                    None,
                    true,
                )],
            };

            for ev in tmp_events {
                match ev {
                    Ok(ev) => events.push(ev),
                    Err(err) => panic!("{}", err),
                }
            }
        }
    }

    if let Some(custom_events) = custom_events {
        for (ev_in, evs_out, val_out) in custom_events {
            let mut note_bang = false;
            let event: Option<Event> = match ev_in[0] {
                0x90 | 0x80 => {
                    let name: Option<String> = match ev_in[1] {
                        0x00..=0x07 => {
                            note_bang = true;
                            Some("Rec track button #".to_string() + &ev_in[1].to_string())
                        }
                        0x08..=0x0F => {
                            note_bang = true;
                            Some("Solo track button #".to_string() + &(ev_in[1] - 0x07).to_string())
                        }
                        0x10..=0x17 => {
                            note_bang = true;
                            Some("Mute track button #".to_string() + &(ev_in[1] ^ 0x10).to_string())
                        }
                        0x18..=0x1F => {
                            note_bang = true;
                            Some(
                                "Select track button #".to_string()
                                    + &((ev_in[1] ^ 0x10) - 0x07).to_string(),
                            )
                        }
                        0x20..=0x27 => Some(
                            "Pan click track button #".to_string() + &(ev_in[1] ^ 0x20).to_string(),
                        ),
                        0x32 => {
                            note_bang = true;
                            Some("Main track flip button".to_string())
                        }
                        0x68..=0x70 => Some(
                            "Fader Touched #".to_string() + &((ev_in[1] - 0x07) ^ 0x60).to_string(),
                        ),
                        _ => {
                            println!("Unable to generate name for event : {:X?}", ev_in[1]);
                            None
                        }
                    };

                    if name.is_some() {
                        let mut vec_in = vec![ev_in[0], ev_in[1], 0x7F];
                        let mut vec_out: Vec<Vec<u8>> = vec![];

                        for ev_out in evs_out {
                            vec_out.push(ev_out.to_vec());
                        }

                        if ev_in[0] == 0x80 {
                            vec_in[2] = 0x40;
                        }

                        match Event::new(
                            event_idx,
                            name.unwrap(),
                            vec_in,
                            Some(vec_out),
                            *val_out,
                            0,
                            None,
                            note_bang,
                        ) {
                            Ok(ev) => Some(ev),
                            Err(err) => {
                                println!("Unable to create custom events : {}", err);
                                None
                            }
                        }
                    } else {
                        None
                    }
                }
                0xB0 => {
                    let name = (ev_in[1] ^ 0x10).to_string();

                    if ev_in[2] == 0x01 || ev_in[2] == 0x41 {
                        match Event::new(
                            event_idx,
                            name,
                            ev_in.clone(),
                            Some(evs_out.to_vec()),
                            *val_out,
                            0,
                            None,
                            false,
                        ) {
                            Ok(ev) => Some(ev),
                            Err(err) => {
                                println!("Unable to create custom events : {}", err);
                                break;
                            }
                        };
                    };

                    None
                }
                _ => None,
            };

            match event {
                Some(ev) => events.push(ev),
                None => println!("Event not added into queue… {:X?} => {:X?}", ev_in, evs_out),
            }

            event_idx += 1;
        }

        // Fader Ctrl :
        // Pan Click : 0020 -> 0027
        // Pan CCW|CW CC : 0010 -> 0017  0041|0001
        // Pan LED CC : 0030 -> 0037
        // Rec : 0000 -> 0007
        // Solo : 0008 -> 000F
        // Mute : 0010 -> 0017
        // Select : 0018 -> 001F
        // Main Flip : 0032

        // Pitch Bend (Fader)
        // PB # by channel : 00E0->00E8
        // PB On/Off : 0068->0070

        //println!("Sys events build :");
        //for (idx, event) in events.iter().enumerate() {
        //    println!("---- \n Event #{} \n {:?} \n ----", idx, event);
        //}
    } else {
        println!("No custom events ;)");
    }

    Ok(events)
}

pub fn get_note_name(note: u8) -> &'static str {
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

pub fn get_octave(note: u8) -> u8 {
    match note {
        21..=127 => {
            return (note / 12) - 1;
        }
        _ => log::warn!("Note not in standard range !"),
    }

    10
}

pub fn get_channel(cmd: u8) -> u8 {
    ((cmd << 4) >> 4) + 1
}

pub fn process_note(cmd: u8, note: u8, vel: u8) -> MidiMesg {
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

pub fn process_cc(cc_num: u8, cc_msb_value: u8, cc_lsb_value: Option<u8>) -> MidiMesg {
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

pub fn process_sys(event: &[u8]) {
    println!("SysEx not implemented yet !");

    if !event.contains(&0xF7) {
        log::warn!("SysEx send without tail !");
    }

    let _end_pos = event.iter().position(|&x| x == 0xF7);
}

pub fn process_pitch_bend(pitch: (u8, u8)) -> MidiMesg {
    let mut midi_mesg = MidiMesg::new();

    let (lsb, msb) = pitch;
    let u16_pitch = (msb as u16) << 7 | lsb as u16;
    let norm_pitch = (u16_pitch as f32) / 16384.0;
    println!("Pitch bend values : {}", norm_pitch);

    midi_mesg.name = "Pitch bend".to_string();
    midi_mesg.value = norm_pitch;

    midi_mesg
}
