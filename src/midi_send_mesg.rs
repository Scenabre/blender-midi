use rainout::{MAX_MIDI_MSG_SIZE, RawMidi, ProcessInfo};
//use crate::midi_process_mesg::MidiMesg;

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
    let u16_value = (value*16384.0).round() as u16;

    [(u16_value >> 8) as u8,(u16_value & 0xff) as u8] // [high_byte, low_byte] => [msb,lsb]
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

pub fn send_midi_mesg(mesg: &[RawMidi]) -> bool {
    println!("Empty fn");
    return false
}
