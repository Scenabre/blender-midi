use rainout::RawMidi;
pub type MidiResult = Result<u8, &'static str>;
const CHROM_RANGE: [&str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];

pub fn process_midi_mesg(events: &[RawMidi]) -> MidiResult {

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
    
    fn get_note_name(note: u8) -> &'static str {
        if note < 21 || note > 108 {
            log::warn!("Note not in standard range !");
        }

        let note_idx: usize = ((note-21)%12).into();

       return CHROM_RANGE[note_idx]; 
    }

    fn get_octave(note: u8) -> u8 {
        if note < 21 || note > 108 {
            log::warn!("Note not in standard range !");
        }
        return (note/12)-1;

    }

    fn get_channel(cmd: u8) -> u8 {
        return ((cmd << 4) >> 4)+1;
    }

    fn convert_half(vel: u8) -> f32 {
        return vel as f32 / 127.0;
    }

    fn process_note(cmd: u8, note: u8, vel: u8) {
        if cmd == 0x80 || vel == 0 {
            println!("Note off : {}{}", get_note_name(note),get_octave(note));
            return;
        }

        println!("Note on : {}{} (vel: {})",get_note_name(note),get_octave(note),convert_half(vel));

    }

    fn process_cc(cc_num: u8, cc_msb_value: u8, cc_lsb_value: Option<u8>) {

        // CC numbers :
        // 0x00 : Bank change
        // 0x01 : Modulation depth
        // 0x02..0x03 : Not used
        // 0x04 : Foot ctrl
        // 0x05 : Portamento
        // 0x06 : Not used
        // 0x07 : Channel Volume
        //

        let cc_msb_range: Vec<u8> = vec![0,1,2,4,5,6,7,8,10,11,12,13,16,17,18,19,98,100];
        let cc_lsb_range: Vec<u8> = vec![32,33,34,36,37,38,39,40,42,43,44,45,48,49,50,51,99,101];

        fn print_cc_value(cc_num: u8, cc_value: u16) {

            match cc_num {
                0x00 => println!("CC not used in Blender Midi"),
                0x01 => println!("Modulation wheel : {}", cc_value),
                0x05 => println!("Portamento : {}", cc_value),
                _ =>  println!("CC #{} : {}", cc_num, cc_value),
            }

        }

        match cc_lsb_value {
            None => {
                print_cc_value(cc_num, cc_msb_value as u16);
            },
            Some(cc_lsb_value) => {
                if cc_lsb_value > 0x65 {
                    log::error!("CC value superior to 101 (0x65)");
                    return;
                }

                let u16_cc_value = (cc_msb_value as u16) << 7 | cc_lsb_value as u16;

                println!("CC with lsb : {}", u16_cc_value);

            },
        };
    }

    fn process_sys(event : &[u8]) {
        let _cmd = event[0];
    }

    fn process_pitch_bend(pitch: (u8,u8)) {
        let (lsb,msb) = pitch;
        let u16_pitch = (msb as u16) << 7 | lsb as u16;
        let norm_pitch = (u16_pitch as f32)/16384.0;
        println!("Pitch bend values : {}", norm_pitch);
    }

    let mut cc_lsb_flag = false;
    let mut cc_msb_val_save: u8 = 0;

    for event in events.iter() {
        let event = event.data();

        let cmd = event[0];

        if cmd == 0xFF {
            return Err("User press PANIC on midi device, shutdown client !");
        }

        println!("CHANNEL : {}", get_channel(cmd));

        let clean_cmd = (cmd >> 4) << 4;

        println!("Raw MIDI : {:04X?} {:?}", event, event);

        match clean_cmd {
            0x80|0x90 => process_note(clean_cmd, event[1], event[2]),
            0xA0 => println!("Poly Key Pressure Aftertouch on {}{} : {}", get_note_name(event[1]), get_octave(event[1]), convert_half(event[2])),
            0xB0 => {
                match event[2] {
                    cc_msb_value if cc_msb_value <= 0x1F => {
                        cc_lsb_flag = true;
                        cc_msb_val_save = cc_msb_value;
                        process_cc(event[1],cc_msb_value,None);
                    },
                    cc_lsb_value if cc_lsb_value > 0x1F => {
                        if cc_lsb_value == 0x01 || cc_lsb_value == 0x41 {
                            process_cc(event[1],cc_lsb_value, None);
                        } else if cc_lsb_flag == false {
                            log::warn!("Find CC LSB besfore MSB !");
                        } else {
                            println!("CC LSB find : {}", cc_lsb_value);
                            process_cc(event[1],cc_msb_val_save,Some(cc_lsb_value));
                        }
                    },
                    _ => log::warn!("Unknown value found for CC !"),
                }
            },
            0xC0 => println!("Command not used in Blender Midi : {:04X?}", cmd),
            0xD0 => println!("Channel Pressure Aftertouch : {}", convert_half(event[1])),
            0xE0 => process_pitch_bend((event[1],event[2])),
            0xF0 => process_sys(event),
            _ => log::warn!("Unkown event : {:04X?}", event),
        }

    }

    Ok(0)
}
