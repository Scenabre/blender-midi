use rainout::{MAX_MIDI_MSG_SIZE, RawMidi, ProcessInfo};
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


pub fn convert_value_to_lsb_msb(value: f32) -> [u8;2] {

    match value {
        value if value == 0.0 => return [0,0],
        value if value == 1.0 => return [0x7C,0x7F],
        _ => {

            let u16_value = (value*16384.0).round() as u16;
            let lsb_value = ((u16_value & 0xff)) as u8;
            let msb_value = (u16_value >> 7) as u8;

            let u16_reverse_value = (msb_value as u16) << 7 | lsb_value as u16;
            println!("Calculate CC value : {}", (u16_reverse_value as f32)/16384.0);

            return [lsb_value,msb_value] 
        }
    }
}

pub fn make_raw_midi_mesg(proc_info: &ProcessInfo, mesg: [u8;MAX_MIDI_MSG_SIZE]) -> Result<RawMidi, String> {
    // Note off = NOFF
    // Note on = NON
    // Aftertouch = AT
    // Con CTRL = CC
    // Chan press = CP
    // Pitch bend = PB
    let raw_midi_mesg: Result<RawMidi,String> = match RawMidi::new(proc_info.frames.try_into().unwrap(),&mesg) {
        Ok(raw_midi) => Ok(raw_midi),
        Err(err) =>  {
            log::error!("Unable to make RawMidi from data : {:?} (err: {})", mesg, err);
            Err("Unable to make RawMidi from data !".to_string())
        },
    };

    return raw_midi_mesg
}

fn make_raw_midi_mesg_fast(time: u32, mesg: [u8;MAX_MIDI_MSG_SIZE]) -> Result<RawMidi, String>
{
    let raw_midi_mesg: Result<RawMidi,String> = match RawMidi::new(time,&mesg) {
        Ok(raw_midi) => Ok(raw_midi),
        Err(err) =>  {
            log::error!("Unable to make RawMidi from data : {:?} (err: {})", mesg, err);
            Err("Unable to make RawMidi from data !".to_string())
        },
    };

    return raw_midi_mesg

}

fn make_sysex_mesg(time: u32, lcd_num: u8, line_num: u8, mesg: String) -> Result<RawMidi, String> {

    // SYSEX MATRIX POS
    // 00 38
    // 07 3F
    // 0E 46
    // 15 4D
    // 1C 54
    // 23 5B
    // 2A 62
    // 31 69
    //

    let position: u8 = ((lcd_num-1)+((line_num-1)*8))*7;

    let mut midi_data: [u8;MAX_MIDI_MSG_SIZE] = [0;MAX_MIDI_MSG_SIZE];

    let prefix = [0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, position];

    let content = mesg.into_bytes();

    let mesg_len = content.len();

    midi_data[0..7].copy_from_slice(&prefix);

    midi_data[7..(7+mesg_len)].copy_from_slice(&content);

    midi_data[7+mesg_len] = 0xF7;

    let raw_midi_mesg = make_raw_midi_mesg_fast(time,midi_data).unwrap();

    return Ok(raw_midi_mesg)

}

pub fn send_midi_mesg(mesg: &[RawMidi]) -> bool {
    println!("Empty fn");
    return false
}

pub fn initialize_mc_device() -> Result<Vec<RawMidi>,String> {
    println!("Initilizing midi device : ");
    let mut raw_midi_mesg: Vec<RawMidi> = Vec::new();
    let mut midi_mesg: [u8;MAX_MIDI_MSG_SIZE] = [0;MAX_MIDI_MSG_SIZE];

    let pitch_bend_prefix = 0xE0;

    let mut time: u32 = 0;

    //let sysx_mesg = [0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x00, 0x41, 0x78, 0x65, 0x6C, 0xF7, 0, 0, 0, 0];
    //let sysx_mesg_2 = [0xF0, 0x00, 0x00, 0x66, 0x14, 0x12, 0x07, 0x41, 0x78, 0x65, 0x6C, 0xF7, 0, 0, 0, 0];

    //raw_midi_mesg.push(make_raw_midi_mesg_fast(time,sysx_mesg).unwrap());
    //raw_midi_mesg.push(make_raw_midi_mesg_fast(time,sysx_mesg_2).unwrap());
    


    for pb_idx in 0..11 {
        let pb_num = pitch_bend_prefix + pb_idx;
        let value_out = convert_value_to_lsb_msb(1.0);
        midi_mesg[0] = pb_num;
        midi_mesg[1..3].copy_from_slice(&value_out);

        match make_raw_midi_mesg_fast(time,midi_mesg) {
            Ok(raw_midi) => raw_midi_mesg.push(raw_midi),
            Err(..) => { 
                log::error!("Unable to trigger event abort");
                //Err("Unable to initialize mc device !".to_string())
            },
        };

        //time += 100;
    }

    raw_midi_mesg.push(make_sysex_mesg(time+1000,1,1,"TEST".to_string()).unwrap());

    return Ok(raw_midi_mesg)

}
