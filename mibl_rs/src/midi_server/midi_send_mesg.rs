use crate::midi_server::container::{RawMidi, MAX_MIDI_MSG_SIZE};
use crate::midi_server::math_utils::split_digits;
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

pub fn make_raw_midi_mesg(stamp: &u64, mesg: &Vec<u8>) -> Result<RawMidi, String> {
    // Note off = NOFF
    // Note on = NON
    // Aftertouch = AT
    // Con CTRL = CC
    // Chan press = CP
    // Pitch bend = PB

    let raw_midi_mesg: Result<RawMidi, String> = match RawMidi::new(*stamp, mesg) {
        Ok(raw_midi) => Ok(raw_midi),
        Err(err) => {
            log::error!(
                "Unable to make RawMidi from data : {:?} (err: {})",
                mesg,
                err
            );
            Err("Unable to make RawMidi from data !".to_string())
        }
    };

    raw_midi_mesg
}

fn make_raw_midi_mesg_fast(stamp: &u64, mesg: &Vec<u8>) -> Result<RawMidi, String> {
    let raw_midi_mesg: Result<RawMidi, String> = match RawMidi::new(*stamp, mesg) {
        Ok(raw_midi) => Ok(raw_midi),
        Err(err) => {
            log::error!(
                "Unable to make RawMidi from data : {:?} (err: {})",
                mesg,
                err
            );
            Err("Unable to make RawMidi from data !".to_string())
        }
    };

    raw_midi_mesg
}

fn make_sysex_mesg(time: u64, lcd_num: u8, line_num: u8, mesg: String) -> Result<RawMidi, String> {
    // SYSEX MATRIX POS
    // 00 38
    // 07 3F
    // 0E 46
    // 15 4D
    // 1C 54
    // 23 5B
    // 2A 62
    // 31 69

    assert!((1..8).contains(&lcd_num));
    assert!((1..2).contains(&line_num));

    let position: u8 = ((lcd_num - 1) + ((line_num - 1) * 8)) * 7;

    let mut midi_data: Vec<u8> = Vec::new();

    let mut prefix = vec![0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, position];

    let content = mesg.into_bytes();

    midi_data.append(&mut prefix);

    midi_data.extend(content);

    midi_data.push(0xF7);

    let raw_midi_mesg = make_raw_midi_mesg_fast(&time, &midi_data).unwrap();

    Ok(raw_midi_mesg)
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
        match make_raw_midi_mesg(&stamp, &vec![0xB0, digit as u8, (*value + 0x30)]) {
            Ok(raw_midi) => raw_midi_timestamp.push(raw_midi),
            Err(err) => println!("Unable to create message for timestamp ! {}", err),
        }
    }

    Ok(raw_midi_timestamp)
}

pub fn assign_gen(assign: u8) -> Result<RawMidi, String> {
    todo!()
}

pub fn reset_mc_device() -> Result<Vec<RawMidi>, String> {
    todo!()
}

pub fn initialize_mc_device() -> Result<Vec<RawMidi>, String> {
    log::info!("Initilizing midi device : ");
    let mut raw_midi_mesg: Vec<RawMidi> = Vec::new();

    let pitch_bend_prefix = 0xE0;

    let time: u64 = 0;

    //let sysx_mesg = [0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x00, 0x41, 0x78, 0x65, 0x6C, 0xF7, 0, 0, 0, 0];
    //let sysx_mesg_2 = [0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x07, 0x41, 0x78, 0x65, 0x6C, 0xF7, 0, 0, 0, 0];

    //raw_midi_mesg.push(make_raw_midi_mesg_fast(time,sysx_mesg).unwrap());
    //raw_midi_mesg.push(make_raw_midi_mesg_fast(time,sysx_mesg_2).unwrap());

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
                //Err("Unable to initialize mc device !".to_string())
            }
        };

        midi_mesg.clear();

        //time += 100;
    }

    for pan_idx in 0..9 {
        midi_mesg.push(0xB0);
        midi_mesg.push(0x30 + pan_idx);
        midi_mesg.push(0x0D);

        match make_raw_midi_mesg_fast(&time, &midi_mesg) {
            Ok(raw_midi) => {
                log::info!("Initializing pan LED #{}", pan_idx);
                raw_midi_mesg.push(raw_midi);
            }
            Err(..) => {
                log::error!("Unable to trigger event abort");
                //Err("Unable to initialize mc device !".to_string())
            }
        };

        midi_mesg.clear();
    }

    match timestamp_gen(1, 45, 30, 250) {
        Ok(timestamp) => {
            for midi_mesg in timestamp {
                raw_midi_mesg.push(midi_mesg);
            }
        }
        Err(err) => println!("Unable to generate timestamp : {}", err),
    }

    log::info!("Initilizing LCD display #1");
    raw_midi_mesg.push(make_sysex_mesg(time + 1000, 1, 1, "TEST".to_string()).unwrap());
    log::info!("Initilizing LCD display #1");
    raw_midi_mesg.push(make_sysex_mesg(time + 1000, 2, 1, ":)".to_string()).unwrap());

    Ok(raw_midi_mesg)
}
