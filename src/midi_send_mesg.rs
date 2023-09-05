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
    let u16_value = (value*16384.0).round() as u16;
    let lsb_value = ((u16_value & 0xff)) as u8;
    let msb_value = (u16_value >> 7) as u8;

    let u16_reverse_value = (msb_value as u16) << 7 | lsb_value as u16;
    println!("Calculate CC value : {}", (u16_reverse_value as f32)/16384.0);


    [lsb_value,msb_value] 
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

fn make_raw_midi_mesg_fast(mesg: [u8;MAX_MIDI_MSG_SIZE]) -> Result<RawMidi, String>
{
    let raw_midi_mesg: Result<RawMidi,String> = match RawMidi::new(0,&mesg) {
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

pub fn initialize_mc_device() -> Result<Vec<RawMidi>,String> {
    println!("Initilizing midi device : ");
    let mut raw_midi_mesg: Vec<RawMidi> = Vec::new();
    let mut midi_mesg: [u8;MAX_MIDI_MSG_SIZE] = [0;MAX_MIDI_MSG_SIZE];

    let pitch_bend_prefix = 0xE0;

    for pb_idx in 0..9 {
        let pb_num = pitch_bend_prefix + pb_idx;
        let value_out = convert_value_to_lsb_msb(0.8);
        midi_mesg[0] = pb_num;
        midi_mesg[1..3].copy_from_slice(&value_out);

        match make_raw_midi_mesg_fast(midi_mesg) {
            Ok(raw_midi) => raw_midi_mesg.push(raw_midi),
            Err(..) => { 
                log::error!("Unable to trigger event abort");
                //Err("Unable to initialize mc device !".to_string())
            },
        };
    }

    return Ok(raw_midi_mesg)

}
