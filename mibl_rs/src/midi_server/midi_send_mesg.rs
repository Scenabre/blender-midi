use crate::midi_server::container::{RawMidi, MAX_MIDI_MSG_SIZE};
use crate::midi_server::math_utils::split_digits;
use crate::midi_server::sys_event::SYS_EVENT_ARRAY;

//use crate::midi_process_mesg::MidiMesg;
//
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

pub fn convert_value_to_lsb_msb(value: f32) -> (u8, u8) {
    match value {
        value if value == 0.0 => (0, 0),
        value if value == 1.0 => (0x7C, 0x7F),
        _ => {
            let u16_value = (value * 16384.0).round() as u16;
            let lsb_value = (u16_value & 0xff) as u8;
            let msb_value = (u16_value >> 7) as u8;

            let u16_reverse_value = (msb_value as u16) << 7 | lsb_value as u16;
            println!(
                "Calculate CC value : {}",
                (u16_reverse_value as f32) / 16384.0
            );

            (lsb_value, msb_value)
        }
    }
}

pub fn make_raw_midi_mesg(stamp: &u64, mesg: &[u8]) -> Result<RawMidi, String> {
    // Note off = NOFF
    // Note on = NON
    // Aftertouch = AT
    // Con CTRL = CC
    // Chan press = CP
    // Pitch bend = PB

    let raw_midi_mesg: Result<RawMidi, String> = match RawMidi::new(*stamp, mesg) {
        Ok(raw_midi) => Ok(raw_midi),
        Err(err) => {
            //log::error!(
            //    "Unable to make RawMidi from data : {:?} (err: {})",
            //    mesg,
            //    err
            //);
            Err("Unable to make RawMidi from data : ".to_string() + &err.to_string())
        }
    };

    raw_midi_mesg
}

fn make_raw_midi_mesg_fast(stamp: &u64, mesg: &[u8]) -> Result<RawMidi, String> {
    let raw_midi_mesg: Result<RawMidi, String> = match RawMidi::new(*stamp, mesg) {
        Ok(raw_midi) => Ok(raw_midi),
        Err(err) => {
            //log::error!(
            //    "Unable to make RawMidi from data : {:?} (err: {})",
            //    mesg,
            //    err
            //);
            Err("Unable to make RawMidi from data : ".to_string() + &err.to_string())
        }
    };

    raw_midi_mesg
}

pub fn make_lcd_mesg(
    time: u64,
    lcd_num: u8,
    line_num: u8,
    mesg: String,
) -> Result<RawMidi, String> {
    // SYSEX MATRIX POS
    // 00 38
    // 07 3F
    // 0E 46
    // 15 4D
    // 1C 54
    // 23 5B
    // 2A 62
    // 31 69

    assert!((1..=9).contains(&lcd_num));
    assert!((1..=2).contains(&line_num));

    let position: u8 = ((lcd_num - 1) + ((line_num - 1) * 8)) * 7;

    let mut midi_data: Vec<u8> = Vec::new();

    let mut prefix = vec![0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, position];

    let content = mesg.into_bytes();

    midi_data.append(&mut prefix);

    midi_data.extend(content);

    midi_data.push(0xF7);

    make_raw_midi_mesg(&time, &midi_data)
}

pub fn gen_lcd_string(stamp: u64, mesg: Option<String>) -> Result<Vec<RawMidi>, String> {
    let mut raw_midi_mesg: Vec<RawMidi> = Vec::new();

    fn line_append(
        vec: &mut Vec<RawMidi>,
        stamp: &u64,
        line: &[u8],
        line_pos: u8,
    ) -> Result<(), String> {
        let chunk_size = 7;
        let line_len = if line.len() <= 110 { line.len() } else { 110 };

        let loop_iter = if line_len / chunk_size <= 8 {
            line_len / chunk_size
        } else {
            8
        };

        let loop_rem = line_len % chunk_size;

        for idx in 0..=loop_iter {
            let min = idx * chunk_size;
            let mut max = min + loop_rem;

            if idx != loop_iter {
                max = min + chunk_size;
            }

            match String::from_utf8(line[min..max].to_vec()) {
                Ok(str) => {
                    match make_lcd_mesg(*stamp, (idx as u8) + 1, line_pos, str) {
                        Ok(raw_midi) => vec.push(raw_midi),
                        Err(err) => return Err(err),
                    };
                }
                Err(err) => return Err(err.to_string()),
            }
        }
        Ok(())
    }

    match mesg {
        Some(mesg) => {
            let mesg_str = mesg.as_bytes();
            let mesg_len = mesg_str.len();

            if mesg_len > 55 {
                match mesg_str.split_at_checked(56) {
                    Some((line1, line2)) => {
                        match line_append(&mut raw_midi_mesg, &stamp, line1, 1) {
                            Ok(()) => (),
                            Err(err) => return Err(err),
                        }

                        match line_append(&mut raw_midi_mesg, &stamp, line2, 2) {
                            Ok(()) => (),
                            Err(err) => return Err(err),
                        }
                    }
                    None => return Err("Unable to slice the string in two :(".to_string()),
                };
            } else if mesg_len <= 55 {
                match line_append(&mut raw_midi_mesg, &stamp, mesg_str, 1) {
                    Ok(()) => (),
                    Err(err) => return Err(err),
                }
            } else {
                return Err("Error when evaluated String length… ".to_string());
            };

            Ok(raw_midi_mesg)
        }
        None => {
            let empty_str = [0x10; 112];

            match line_append(&mut raw_midi_mesg, &stamp, &empty_str[0..=55], 1) {
                Ok(()) => (),
                Err(err) => return Err(err),
            }

            match line_append(&mut raw_midi_mesg, &stamp, &empty_str[56..112], 2) {
                Ok(()) => (),
                Err(err) => return Err(err),
            }
            Ok(raw_midi_mesg)
        }
    }
}

pub fn send_midi_mesg(mesg: &[RawMidi]) -> bool {
    println!("Empty fn");
    false
}

pub fn timestamp_gen(
    hours: usize,
    minutes: usize,
    seconds: usize,
    frames: usize,
) -> Result<Vec<RawMidi>, String> {
    let mut raw_midi_timestamp: Vec<RawMidi> = Vec::new();
    let stamp: u64 = 0;
    // frames : 000 -> 3E7
    // sec 00 -> 3C
    // min 00 -> 3C
    // hours 000 -> 3EC

    let frames = split_digits(&frames, 3);
    let seconds = split_digits(&seconds, 2);
    let minutes = split_digits(&minutes, 2);
    let hours = split_digits(&hours, 3);

    let mut timestamp: Vec<u8> = vec![];
    timestamp.extend(frames);
    timestamp.extend(seconds);
    timestamp.extend(minutes);
    timestamp.extend(hours);

    for (idx, value) in timestamp.iter().enumerate() {
        let digit = idx + 0x40;
        match make_raw_midi_mesg(&stamp, &[0xB0, digit as u8, (*value + 0x30)]) {
            Ok(raw_midi) => raw_midi_timestamp.push(raw_midi),
            Err(err) => println!("Unable to create message for timestamp ! {}", err),
        }
    }

    Ok(raw_midi_timestamp)
}

pub fn assign_gen(assign: usize) -> Result<Vec<RawMidi>, String> {
    let split_assign = split_digits(&assign, 2);
    let mut raw_midi_mesg: Vec<RawMidi> = Vec::new();

    match make_raw_midi_mesg(&0, &[0xB0, 0x4A, split_assign[0] + 0x30]) {
        Ok(raw_midi) => raw_midi_mesg.push(raw_midi),
        Err(err) => {
            return Err(
                "Unable to generate assign digit midi mesg : ".to_string() + &err.to_string()
            )
        }
    }

    match make_raw_midi_mesg(&0, &[0xB0, 0x4B, split_assign[1] + 0x30]) {
        Ok(raw_midi) => raw_midi_mesg.push(raw_midi),
        Err(err) => return Err(err.to_string()),
    }

    Ok(raw_midi_mesg)
}

pub fn pan_knob_gen(mode: u8, knob_num: u8, knob_value: u8) -> Result<RawMidi, String> {
    let cmd = 0xB0;
    let midi_knob_num = if knob_num < 9 {
        0x30 + (knob_num - 1)
    } else {
        return Err("Knob number superior to 8 !".to_string());
    };
    if knob_value > 0x0B {
        return Err("Knob value must be inferior or equal to 11 (0x0B)".to_string());
    }
    if mode > 0x03 {
        return Err("Knob mode must be inferior or equel to 3 (0x03)".to_string());
    }

    let clamped_value = if mode == 0x03 && knob_value > 0x06 {
        0x06
    } else {
        knob_value
    };

    let midi_knob_value = (mode << 4) | clamped_value;

    make_raw_midi_mesg(&0, &[cmd, midi_knob_num, midi_knob_value])
}

pub fn meter_led(meter_num: u8, sound_value: i8, clip: bool) -> Result<RawMidi, String> {
    //0xsC 	(0 dB) 	Red (clip)
    //0xsB 	(>= -2 dB) 	Yellow
    //0xsA 	(>= -4 dB) 	Yellow
    //0xs9 	(>= -6 dB) 	Yellow
    //0xs8 	(>= -8 dB) 	Green
    //0xs7 	(>= -10 dB) 	Green
    //0xs6 	(>= -14 dB) 	Green
    //0xs5 	(>= -20 dB) 	Green
    //0xs4 	(>= -30 dB) 	Green
    //0xs3 	(>= -40 dB) 	Green
    //0xs2 	(>= -50 dB) 	Green
    //0xs1 	(>= -60 dB) 	Green
    //0xs0 	0 % (< -60 dB) 	All LEDs Off
    // s [0..=7]

    let midi_sound_value = match sound_value {
        0 => 0x0C,
        v if v > 0 => {
            if clip {
                0x0E
            } else {
                0x0D
            }
        }
        v if v >= -2 => 0x0B,
        v if v >= -4 => 0x0A,
        v if v >= -6 => 0x09,
        v if v >= -8 => 0x08,
        v if v >= -10 => 0x07,
        v if v >= -14 => 0x06,
        v if v >= -20 => 0x05,
        v if v >= -30 => 0x04,
        v if v >= -40 => 0x03,
        v if v >= -50 => 0x02,
        v if v >= -60 => 0x01,
        v if v < -60 => 0x00,
        _ => 0x00,
    };

    if meter_num > 7 {
        let midi_meter_value = (7 << 4) | midi_sound_value;
        return make_raw_midi_mesg(&0, &[0xD0, midi_meter_value]);
    }

    let midi_meter_value = (meter_num << 4) | midi_sound_value;

    make_raw_midi_mesg(&0, &[0xD0, midi_meter_value])
}

pub fn send_note_bang(note: u8, led_value: u8) -> Result<Vec<RawMidi>, String> {
    let stamp = 0;

    if led_value <= 127 {
        let on_mesg = vec![0x90, note, led_value];
        let off_mesg = vec![0x80, note, 0x40];
        let mut raw_midi_mesg: Vec<RawMidi> = Vec::with_capacity(2);

        match make_raw_midi_mesg(&stamp, &on_mesg) {
            Ok(res) => raw_midi_mesg.push(res),
            Err(err) => return Err(err.to_string()),
        };

        match make_raw_midi_mesg(&stamp, &off_mesg) {
            Ok(res) => raw_midi_mesg.push(res),
            Err(err) => return Err(err.to_string()),
        };

        return Ok(raw_midi_mesg);
    }

    Err(format!("Bad led mode {} : 0 or any even value to LED Off, 1 or any odd value to LED Blink, 127 to LED On", led_value))
}

pub fn initialize_mc_device() -> Result<Vec<RawMidi>, String> {
    log::info!("Initilizing midi device : ");
    let mut raw_midi_mesg: Vec<RawMidi> = Vec::new();

    let pitch_bend_prefix = 0xE0;

    let time: u64 = 0;

    let mut midi_mesg: Vec<u8> = Vec::with_capacity(MAX_MIDI_MSG_SIZE);

    for pb_idx in 0..9 {
        let pb_num = pitch_bend_prefix + pb_idx;

        let (lsb, msb) = convert_value_to_lsb_msb(0.75);

        midi_mesg.push(pb_num);
        midi_mesg.push(lsb);
        midi_mesg.push(msb);

        match make_raw_midi_mesg_fast(&time, &midi_mesg) {
            Ok(raw_midi) => {
                log::info!("Initializing slider #{}", pb_idx);
                raw_midi_mesg.push(raw_midi);
            }
            Err(..) => {
                log::error!("Unable to trigger event abort");
            }
        };

        midi_mesg.clear();
    }

    for pan_idx in 1..9 {
        match pan_knob_gen(3, pan_idx, 0x06) {
            Ok(raw_midi) => raw_midi_mesg.push(raw_midi),
            Err(err) => println!("Unable to create Pan Knob midi message : {}", err),
        }
    }

    match timestamp_gen(1, 45, 30, 250) {
        Ok(timestamp) => {
            for midi_mesg in timestamp {
                raw_midi_mesg.push(midi_mesg);
            }
        }
        Err(err) => println!("Unable to generate timestamp : {}", err),
    }

    // Default assignment digit
    match assign_gen(55) {
        Ok(vec_raw_midi) => {
            for raw_midi in vec_raw_midi {
                raw_midi_mesg.push(raw_midi);
            }
        }
        Err(err) => println!("Unable to generate assign digit : {}", err),
    }

    let test_mesg = "She bath'd with roses red, and violets blue, And all the sweetest flowres, that in the forrest grew.".to_string();

    match gen_lcd_string(time, Some(test_mesg)) {
        Ok(vec_raw_midi) => {
            for raw_midi in vec_raw_midi {
                raw_midi_mesg.push(raw_midi);
            }
        }
        Err(err) => println!("Unable to generate lcd string : {}", err),
    }

    for meter in 0..=7 {
        match meter_led(meter, 0, false) {
            Ok(raw_midi) => raw_midi_mesg.push(raw_midi),
            Err(err) => println!("Unable to generate meter midi mesg : {}", err),
        }
    }

    Ok(raw_midi_mesg)
}

pub fn reset_mc_device() -> Result<Vec<RawMidi>, String> {
    let pitch_bend_prefix = 0xE0;
    let time: u64 = 0;
    let mut raw_midi_mesg: Vec<RawMidi> = Vec::new();
    let mut midi_mesg: Vec<u8> = Vec::with_capacity(MAX_MIDI_MSG_SIZE);

    for (idx, (value, name)) in SYS_EVENT_ARRAY.iter().enumerate() {
        if *value != 0x3C {
            match send_note_bang(*value, 0x00) {
                Ok(bang) => {
                    println!("Resetting System LED #{} : {}", idx + 1, name);
                    raw_midi_mesg.extend(bang);
                }
                Err(err) => return Err(err),
            }
        }
    }

    for pb_idx in 0..9 {
        let pb_num = pitch_bend_prefix + pb_idx;

        let (lsb, msb) = convert_value_to_lsb_msb(0.0);

        midi_mesg.push(pb_num);
        midi_mesg.push(lsb);
        midi_mesg.push(msb);

        match make_raw_midi_mesg_fast(&time, &midi_mesg) {
            Ok(raw_midi) => {
                log::info!("Resetting slider #{}", pb_idx);
                raw_midi_mesg.push(raw_midi);
            }
            Err(..) => {
                log::error!("Unable to trigger event abort");
            }
        };

        midi_mesg.clear();
    }

    match timestamp_gen(0, 0, 0, 0) {
        Ok(timestamp) => {
            for midi_mesg in timestamp {
                raw_midi_mesg.push(midi_mesg);
            }
        }
        Err(err) => println!("Unable to generate timestamp : {}", err),
    }

    Ok(raw_midi_mesg)
}
