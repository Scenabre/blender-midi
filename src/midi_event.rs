use rainout::{RawMidi, MAX_MIDI_MSG_SIZE};
use crate::midi_process_mesg::MidiMesg;
use crate::midi_send_mesg::{make_raw_midi_mesg, send_midi_mesg};

const SIZE: usize = 16;
const EPSILON: f32 = 0.01;

#[derive(Debug)]
struct Event {
    name : String,
    mesg_in: [u8;MAX_MIDI_MSG_SIZE],
    //mesg_out: [u8;MAX_MIDI_MSG_SIZE],
    //mod_rule: // 0 in->out, 1 in+x = out, 2 in-x = out,
    //mod_amount: // Increase/Descrease by
}

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

pub fn trigger_midi_events(mesg: &[u8]) -> bool {

    //let cc_60_cw_event: Event = Event {
    //    name : "CC #60 CCW".to_string(),
    //    mesg_in: [0xB0,0x3C,0x41],
    //    mod_rule: 0,
    //    mod_amount: 0.01,
    //    mesg_out: [0xE0]
    //}

    //let value_cmp = cc_60_cw_event.mesg_in.value;

    //if mesg.name == cc_60_cw_event.mesg_in.name && value_cmp-EPSILON < mesg.value && mesg.value < value_cmp+EPSILON {
    //    println!("Event trigger :)");
        //make_raw_midi_mesg()
    //}

    return false
}
