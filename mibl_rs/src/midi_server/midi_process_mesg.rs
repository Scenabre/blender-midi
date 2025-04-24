use crate::midi_server::container::{
    Event, ExtTrigger, MidiMesg, MidiProcess, MidiResult, RawMidi, SIGflag, TriggerResult,
    MAX_MIDI_MSG_SIZE,
};
use crate::midi_server::midi_event::{
    get_channel, get_note_name, get_octave, process_cc, process_note, process_pitch_bend,
    process_sys,
};
use crate::midi_server::midi_send_mesg::make_raw_midi_mesg;
use crate::node_utils::sys_event::convert_half;

pub fn process_midi_mesg(
    event: &RawMidi,
    protocole: &str,
    sig_flag: &mut SIGflag,
    triggers: &Option<Vec<Event>>,
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

    let proto = match protocole {
        "HUI" => 0,
        "MC" | "Mackie Control" | "MackieControl" => 1,
        _ => {
            log::warn!("Protocole unknown drop to MC");
            1
        }
    };

    let display_event = event.data();
    let debug = sig_flag.debug;

    if debug {
        println!(
            "\n ---------\n  Midi event to process ({}:{}) : {:04X?}\n ---------\n",
            proto, protocole, display_event
        );

        println!("Delta frames : {:?}", event.delta_frames);
    }

    let event_data = event.data();

    let cmd = event_data[0];

    if cmd == 0xFF {
        return Err("User press PANIC on midi device !");
    }

    let mut val_out = None;

    let channel: u8 = get_channel(cmd);
    let clean_cmd = (cmd >> 4) << 4;

    if debug {
        println!("CHANNEL : {}", channel);

        println!("Raw MIDI : {:04X?} {:?}", event_data, event_data);
    }

    let (midi_mesg_to_send, debug_midi_mesg): (TriggerResult, Option<MidiMesg>) = match triggers {
        Some(triggers) => {
            let mut ext_trigger_result: Option<Vec<ExtTrigger>> = None;
            let mut int_trigger_result: Option<Vec<RawMidi>> = None;
            let mut midi_mesg: Option<MidiMesg> = None;

            for (idx, trigger) in triggers.iter().enumerate() {
                let trigger_mesg_in = trigger.get_mesg_in();
                let mut note_bang = false;

                if trigger_mesg_in.len() >= 3
                    && trigger.get_bang_signal()
                    && sig_flag.note_bang
                    && trigger_mesg_in[1] == sig_flag.note_bang_value
                {
                    note_bang = true;
                }

                if note_bang || event_data == trigger_mesg_in {
                    if debug {
                        println!(
                            "Event triggered {} : {}",
                            trigger.get_index(),
                            trigger.get_name()
                        );
                        println!("Note bang status : {}", note_bang);
                    }

                    let trigger_val_out = trigger.get_val_out();

                    if trigger_val_out.is_none() {
                        match clean_cmd {
                            0x80 | 0x90 => {
                                let mut note_value = event_data[2];

                                if note_bang {
                                    note_value = trigger_mesg_in[5];
                                }

                                let mut tmp_midi_mesg =
                                    process_note(clean_cmd, event_data[1], note_value);

                                tmp_midi_mesg.channel = channel;
                                val_out = Some(tmp_midi_mesg.value);

                                if debug {
                                    println!("{}", tmp_midi_mesg.name);
                                    midi_mesg = Some(tmp_midi_mesg);
                                }
                            }
                            0xA0 => {
                                let poly_key_value = convert_half(event_data[2]);
                                val_out = Some(poly_key_value);

                                if debug {
                                    let tmp_midi_mesg = MidiMesg {
                                        channel,
                                        name: format!(
                                            "Poly Key Pressure Aftertouch on {}{}",
                                            get_note_name(event_data[1]),
                                            get_octave(event_data[1])
                                        ),
                                        value: poly_key_value,
                                    };
                                    println!("{} : {}", tmp_midi_mesg.name, tmp_midi_mesg.value);
                                    midi_mesg = Some(tmp_midi_mesg);
                                }
                            }
                            0xB0 => {
                                let mut cc_flag = sig_flag.cc_flag;
                                match event_data[1] {
                                    cc_num if cc_num > 0x3F && cc_num < 0x62 => {
                                        let tmp_midi_mesg = process_cc(cc_num, event_data[2], None);
                                        val_out = Some(tmp_midi_mesg.value);

                                        if debug {
                                            midi_mesg = Some(tmp_midi_mesg);
                                        }
                                    }
                                    cc_num if cc_num <= 0x1F && !cc_flag.cc_lsb_flag => {
                                        if channel == cc_flag.cc_channel
                                            && cc_flag.cc_note == event_data[1] + 0x20
                                        {
                                            cc_flag.cc_lsb_flag = true;
                                            cc_flag.cc_msb_value = event_data[2];
                                            cc_flag.cc_num = cc_num;
                                        }

                                        if !cc_flag.cc_lsb_flag {
                                            let tmp_midi_mesg =
                                                process_cc(cc_num, event_data[2], None);
                                            val_out = Some(tmp_midi_mesg.value);

                                            if debug {
                                                midi_mesg = Some(tmp_midi_mesg);
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                    cc_num if cc_num > 0x1F && cc_num < 0x40 => {
                                        let tmp_midi_mesg: Option<MidiMesg>;

                                        if !cc_flag.cc_lsb_flag {
                                            tmp_midi_mesg =
                                                Some(process_cc(cc_num, event_data[2], None));
                                            val_out = Some(tmp_midi_mesg.clone().unwrap().value);
                                        } else {
                                            tmp_midi_mesg = Some(process_cc(
                                                cc_flag.cc_num,
                                                cc_flag.cc_msb_value,
                                                Some(event_data[2]),
                                            ));
                                            val_out = Some(tmp_midi_mesg.clone().unwrap().value);
                                            cc_flag.cc_lsb_flag = false;
                                        }

                                        if debug && tmp_midi_mesg.is_some() {
                                            midi_mesg = tmp_midi_mesg;
                                        }
                                    }
                                    _ => log::warn!("Unknown value found for CC !"),
                                }
                            }
                            0xC0 => println!("Command not used in Blender Midi : {:04X?}", cmd),
                            0xD0 => {
                                let chan_press_value = convert_half(event_data[1]);
                                val_out = Some(chan_press_value);

                                if debug {
                                    println!("Channel Pressure Aftertouch : {}", chan_press_value);

                                    midi_mesg = Some(MidiMesg {
                                        channel,
                                        name: "Channel Pressure Aftertouch".to_string(),
                                        value: chan_press_value,
                                    });
                                }
                            }
                            0xE0 => {
                                let tmp_midi_mesg =
                                    process_pitch_bend((event_data[1], event_data[2]));
                                val_out = Some(tmp_midi_mesg.value);

                                if debug {
                                    midi_mesg = Some(tmp_midi_mesg);
                                }
                            }
                            0xF0 => process_sys(event_data),
                            _ => log::warn!("Unkown event : {:04X?}", event_data),
                        }
                    }

                    if let Some(data) = trigger.get_mesg_data() {
                        let mut int_midi_mesg = Vec::<RawMidi>::with_capacity(MAX_MIDI_MSG_SIZE);
                        for mesg in data {
                            int_midi_mesg
                                .push(make_raw_midi_mesg(event.delta_frames(), mesg).unwrap());
                        }
                        int_trigger_result = Some(int_midi_mesg);
                    }

                    let val_out = match (trigger_val_out, val_out) {
                        (None, Some(val)) => Some(val),
                        (Some(val), None) => Some(val),
                        (Some(val), Some(_)) => Some(val),
                        (None, None) => None,
                    };

                    if val_out.is_some() {
                        let mut ext_midi_mesg = Vec::<ExtTrigger>::with_capacity(MAX_MIDI_MSG_SIZE);
                        ext_midi_mesg.push((*trigger.get_index(), val_out.unwrap()));
                        ext_trigger_result = Some(ext_midi_mesg);
                    }

                    if note_bang {
                        if trigger.get_toggable() {
                            let note_value = trigger.get_mesg_in()[1];
                            let mut mesg = [0x90, note_value, 0x7F];

                            if sig_flag.note_led_on.contains(&note_value) {
                                mesg[2] = 0x00;
                                let note_idx = sig_flag
                                    .note_led_on
                                    .iter()
                                    .position(|&x| x == note_value)
                                    .expect("slice should contain elem");
                                sig_flag.note_led_on.remove(note_idx);
                            } else {
                                sig_flag.note_led_on.push(note_value);
                            }

                            let raw_midi_mesg =
                                make_raw_midi_mesg(event.delta_frames(), &mesg).unwrap();

                            match int_trigger_result {
                                Some(ref mut int_trigger) => {
                                    int_trigger.push(raw_midi_mesg);
                                }
                                None => {
                                    int_trigger_result = Some(vec![raw_midi_mesg]);
                                }
                            }
                        }

                        sig_flag.note_on = false;
                        sig_flag.note_bang = false;
                        sig_flag.note_bang_value = 0;
                    }
                }
            }
            ((int_trigger_result, ext_trigger_result), midi_mesg)
        }
        None => ((None, None), None),
    };

    let midi_to_send: MidiProcess = MidiProcess {
        debug: debug_midi_mesg,
        to_send: midi_mesg_to_send,
    };

    Ok(midi_to_send)
}
