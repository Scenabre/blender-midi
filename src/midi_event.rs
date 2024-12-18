use rainout::{RawMidi, ProcessInfo, MAX_MIDI_MSG_SIZE};
use crate::midi_process_mesg::MidiMesg;
use crate::midi_send_mesg::{convert_value_to_lsb_msb,make_raw_midi_mesg};

const SIZE: usize = 16;
const EPSILON: f32 = 0.01;

#[derive(Debug)]
struct Event {
    index: u8,
    name: String,
    mesg_in: Vec<u8>,
    cmd_out: Option<u8>,
    value_out: Option<Vec<u8>>,
    mod_rule: u8,// 0 in->out, 1 in+x = out, 2 in-x = out,
    mod_amount: Option<f32>// Increase/Descrease by
}

// CHANNEL VOICE MESG
// Command  Meaning      # parameters  param 1      param 2
// 0x80 Note-off    2              key          velocity
// 0x90      Note-on     2              key          velocity
// 0xA0      Aftertouch  2              key          touch
// 0xB0      Cont CTRL   2              ctrl #       ctrl value (0-119)
// 0xC0      Prog chg    2              instr # 	
// 0xD0      Chan Press  1              pressure     X
// 0xE0      Pitch bend  2              lsb (7 bits) msb (7 bits)
// 0xF0     (non-musical commands)

pub fn trigger_midi_events(proc_info: &ProcessInfo, mesg: &[u8]) -> Result<Vec<RawMidi>,String> {

    let cc_60_ccw_event: Event = Event {
        index: 0,
        name: "CC #60 CCW → PB #1".to_string(),
        mesg_in: vec![0xB0,0x3C,0x41],
        cmd_out: Some(0xE0),
        value_out: None,
        mod_rule: 0,
        mod_amount: Some(EPSILON),
    };

    let cc_60_cw_event: Event = Event {
        index: 1,
        name: "CC #60 CW → PB #1".to_string(),
        mesg_in: vec![0xB0,0x3C,0x01],
        cmd_out: Some(0xE0),
        value_out: Some(vec![0,0]),
        mod_rule: 0,
        mod_amount: None,
    };

    let triggers: Vec<&Event> = vec![&cc_60_cw_event,&cc_60_ccw_event];

    let mut trigger_result: Vec<RawMidi> = vec![];

    for trigger in triggers.iter() {

        if mesg == trigger.mesg_in {

            println!("Event triggered : {}", trigger.name);

            let mut midi_mesg: [u8;MAX_MIDI_MSG_SIZE] = [0;MAX_MIDI_MSG_SIZE];

            match trigger.index {
                0 => {
                    let value_out = convert_value_to_lsb_msb(0.8);
                    midi_mesg[0] = cc_60_ccw_event.cmd_out.unwrap_or_default();
                    midi_mesg[1..3].copy_from_slice(&value_out);

                },
                1 => {
                    let value_out = convert_value_to_lsb_msb(0.0);
                    midi_mesg[0] = cc_60_cw_event.cmd_out.unwrap_or_default();
                    midi_mesg[1..3].copy_from_slice(&value_out);
                },
                _ => log::warn!("Trigger event not implemented yet"),
            }

            match make_raw_midi_mesg(&proc_info, midi_mesg) {
                Ok(raw_midi) => trigger_result.push(raw_midi),
                Err(..) => log::error!("Unable to trigger event abort"),
            };
        }
    }

    //let value_cmp = cc_60_cw_event.mesg_in.value;

    //if mesg.name == cc_60_cw_event.mesg_in.name && value_cmp-EPSILON < mesg.value && mesg.value < value_cmp+EPSILON {
    //    println!("Event trigger :)");
        //make_raw_midi_mesg()
    //}

    Ok(trigger_result)
}
