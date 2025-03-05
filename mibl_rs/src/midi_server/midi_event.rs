use std::u8;

use crate::midi_server::container::{Event, RawMidi, MAX_MIDI_MSG_SIZE};
use crate::midi_server::midi_send_mesg::{self, convert_value_to_lsb_msb, make_raw_midi_mesg};
use crate::midi_server::sys_event::SYS_EVENT_ARRAY;

const SIZE: usize = 16;
const EPSILON: f32 = 0.01;

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

pub fn craft_recipe(
    use_sys: &bool,
    custom_events: Option<&Vec<[u8; 3]>>,
) -> Result<Vec<Event>, String> {
    let mut events: Vec<Event> = Vec::new();
    let mut event_idx: u8 = 0;

    if *use_sys {
        for (idx, (value, name)) in SYS_EVENT_ARRAY.iter().enumerate() {
            let event = match *value {
                0x3C => Event::new(
                    event_idx,
                    name.to_string(),
                    vec![],
                    Some(0xB0),
                    vec![].into(),
                    0,
                    None,
                ),
                0x71..=0x73 => Event::new(
                    event_idx,
                    name.to_string(),
                    vec![],
                    Some(0x90),
                    vec![].into(),
                    0,
                    None,
                ),
                _ => Event::new(
                    event_idx,
                    name.to_string(),
                    vec![0x90, *value, 0x7F],
                    Some(0x80),
                    vec![*value, 0x40].into(),
                    0,
                    None,
                ),
            };
            match event {
                Ok(ev) => events.push(ev),
                Err(err) => panic!("{}", err),
            }
            event_idx = idx as u8
        }
    }

    if let Some(custom_events) = custom_events {
        for custom_event in custom_events {
            let event: Option<Event> = match custom_event[0] {
                0x90 => {
                    let name: Option<String> = match custom_event[1] {
                        0x00..=0x07 => {
                            Some("Rec track button #".to_string() + &custom_event[1].to_string())
                        }
                        0x08..=0x0F => Some(
                            "Solo track button #".to_string()
                                + &(custom_event[1] - 0x07).to_string(),
                        ),
                        0x10..=0x17 => Some(
                            "Mute track button #".to_string()
                                + &(custom_event[1] ^ 0x10).to_string(),
                        ),

                        0x18..=0x1F => Some(
                            "Select track button #".to_string()
                                + &((custom_event[1] ^ 0x10) - 0x07).to_string(),
                        ),
                        0x20..=0x27 => Some(
                            "Pan click track button #".to_string()
                                + &(custom_event[1] ^ 0x20).to_string(),
                        ),
                        0x32 => Some("Main track flip button".to_string()),
                        0x68..=0x70 => Some(
                            "Fader Touched #".to_string()
                                + &((custom_event[1] - 0x07) ^ 0x60).to_string(),
                        ),
                        _ => {
                            println!("Unable to generate name for event : {:X?}", custom_event[1]);
                            None
                        }
                    };

                    if name.is_some() {
                        match Event::new(
                            event_idx,
                            name.unwrap(),
                            vec![0x90, custom_event[1], 0x7F],
                            Some(0x80),
                            vec![custom_event[1], 0x40].into(),
                            0,
                            None,
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
                0xB0 => {
                    let name = (custom_event[1] ^ 0x10).to_string();

                    if custom_event[2] == 0x01 || custom_event[2] == 0x41 {
                        match Event::new(
                            event_idx,
                            name,
                            vec![0xB0, custom_event[1], custom_event[2]],
                            Some(0xB0),
                            vec![0xB0, custom_event[1], custom_event[2]].into(),
                            0,
                            None,
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
                None => println!("Event not added into queue… {:X?}", custom_event),
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

pub fn trigger_midi_events(
    stamp: &u64,
    mesg: &[u8],
    triggers: &Vec<Event>,
) -> Result<Option<RawMidi>, String> {
    //let cc_60_ccw_event: Event = Event {
    //    index: 0,
    //    name: "CC #60 CCW → PB #1".to_string(),
    //    mesg_in: vec![0xB0, 0x3C, 0x41],
    //    cmd_out: Some(0xE1),
    //    value_out: None,
    //    mod_rule: 0,
    //    mod_amount: Some(EPSILON),
    //};
    //
    //let cc_60_cw_event: Event = Event {
    //    index: 1,
    //    name: "CC #60 CW → PB #1".to_string(),
    //    mesg_in: vec![0xB0, 0x3C, 0x01],
    //    cmd_out: Some(0xE1),
    //    value_out: Some(vec![0, 0]),
    //    mod_rule: 0,
    //    mod_amount: None,
    //};
    //
    //let triggers: Vec<&Event> = vec![&cc_60_cw_event, &cc_60_ccw_event];

    let mut trigger_result: Option<RawMidi> = None;

    for trigger in triggers.iter() {
        if mesg == trigger.get_mesg_in() {
            println!(
                "Event triggered {} : {}",
                trigger.get_index(),
                trigger.get_name()
            );

            let mut midi_mesg = Vec::<u8>::with_capacity(MAX_MIDI_MSG_SIZE);

            if let Some(data) = trigger.get_mesg_data() {
                midi_mesg.extend(data)
            }

            trigger_result = Some(make_raw_midi_mesg(stamp, &midi_mesg).unwrap());
        }
    }

    //let value_cmp = cc_60_cw_event.mesg_in.value;

    //if mesg.name == cc_60_cw_event.mesg_in.name && value_cmp-EPSILON < mesg.value && mesg.value < value_cmp+EPSILON {
    //    println!("Event trigger :)");
    //make_raw_midi_mesg()
    //}

    Ok(trigger_result)
}
