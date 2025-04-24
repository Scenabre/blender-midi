pub const MAX_MIDI_MSG_SIZE: usize = 16;

pub type Ingredient = (Vec<u8>, Vec<Vec<u8>>, Option<f32>);
pub type Recipe = Vec<Ingredient>;
pub type ExtTrigger = (u64, f32);
pub type WaitData = (u64, Vec<Vec<u8>>);
pub type TriggerResult = (Option<Vec<RawMidi>>, Option<Vec<ExtTrigger>>);

#[derive(Debug, Clone, Copy, Default)]
pub struct CCflag {
    pub cc_lsb_flag: bool,
    pub cc_channel: u8,
    pub cc_num: u8,
    pub cc_msb_value: u8,
    pub cc_note: u8,
}

impl CCflag {
    pub fn new(
        cc_lsb_flag: bool,
        cc_channel: u8,
        cc_num: u8,
        cc_msb_value: u8,
        cc_note: u8,
    ) -> Self {
        Self {
            cc_lsb_flag,
            cc_channel,
            cc_num,
            cc_msb_value,
            cc_note,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SIGflag {
    pub reset_signal: bool,
    pub note_on: bool,
    pub note_bang: bool,
    pub note_bang_value: u8,
    pub note_led_on: Vec<u8>,
    pub note_toggle: u8,
    pub note_need_toggle: bool,
    pub cc_flag: CCflag,
    pub update_recipe: bool,
    pub stop_thread: bool,
    pub use_sys_event: bool,
    pub debug: bool,
}

#[derive(Clone)]
pub struct RawMidi {
    /// The amount of time passed, in frames, relative to the start of the process cycle.
    pub delta_frames: u64, // stamp

    data: Vec<u8>,

    len: u8,
}

impl RawMidi {
    /// Create a new midi message from raw bytes.
    ///
    /// * `delta_frames` - The amount of time passed, in frames, relative to the start of the process cycle.
    /// * `data` - The raw bytes of the midi message.
    ///
    /// This returns an error if the length of `data` is greater than `MAX_MIDI_MSG_SIZE` (16).
    pub fn new(delta_frames: u64, data: &[u8]) -> Result<Self, usize> {
        if data.len() <= MAX_MIDI_MSG_SIZE {
            Ok(Self {
                delta_frames,
                data: data.to_vec(),
                len: data.len() as u8,
            })
        } else {
            Err(data.len())
        }
    }

    /// The raw midi data.
    pub fn data(&self) -> &[u8] {
        &self.data[0..usize::from(self.len)]
    }

    /// Delta frames.
    pub fn delta_frames(&self) -> &u64 {
        &self.delta_frames
    }

    /////Clone data
    //pub fn data_clone(&self) -> Vec<u8> {
    //    self.data.clone()
    //}
    //
    /////Clone delta_frames
    //pub fn delta_frames_clone(&self) -> u64 {
    //    self.delta_frames
    //}
    //
    ///// The size of this MIDI message in bytes.
    //pub fn len(&self) -> usize {
    //    usize::from(self.len)
    //}
    //
    ///// Setter
    //pub fn set(&mut self, delta_frames: u64, data: &[u8]) -> Result<(), usize> {
    //    if data.len() <= MAX_MIDI_MSG_SIZE {
    //        self.delta_frames = delta_frames;
    //        self.data = data.to_vec();
    //        self.len = data.len() as u8;
    //
    //        Ok(())
    //    } else {
    //        Err(data.len())
    //    }
    //}
}

impl std::fmt::Debug for RawMidi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Raw MIDI: {{ delta frames: {}, len: {}, data: {:X?} }}",
            self.delta_frames,
            self.len,
            &self.data[0..usize::from(self.len)]
        )
    }
}

impl Default for RawMidi {
    fn default() -> Self {
        RawMidi {
            delta_frames: 0,
            data: vec![0; MAX_MIDI_MSG_SIZE],
            len: 0,
        }
    }
}

#[derive(Clone)]
pub struct Event {
    index: u64,
    name: String,
    mesg_in: Vec<u8>,
    mesg_out: Option<Vec<Vec<u8>>>,
    ext_value_out: Option<f32>,
    mod_rule: u8,            // 0 in->out, 1 in+x = out, 2 in-x = out,
    mod_amount: Option<f32>, // Increase/Descrease by
    note_bang: bool,
    toggable: bool,
}

impl Event {
    pub fn new(
        index: u64,
        name: String,
        mesg_in: Vec<u8>,
        mesg_out: Option<Vec<Vec<u8>>>,
        ext_value_out: Option<f32>,
        mod_rule: u8,
        mod_amount: Option<f32>,
        note_bang: bool,
        toggable: bool,
    ) -> Result<Event, String> {
        if mod_rule <= 2 {
            Ok(Self {
                index,
                name,
                mesg_in,
                mesg_out,
                ext_value_out,
                mod_rule,
                mod_amount,
                note_bang,
                toggable,
            })
        } else {
            Err("mod_rule must be one of the following value : 0,1,2".to_string())
        }
    }

    pub fn get_index(&self) -> &u64 {
        &self.index
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_mesg_in(&self) -> &Vec<u8> {
        &self.mesg_in
    }

    pub fn get_mesg_data(&self) -> &Option<Vec<Vec<u8>>> {
        &self.mesg_out
    }

    pub fn get_mod_amount(&self) -> Option<f32> {
        match self.mod_amount {
            Some(amount) => match self.mod_rule {
                0 | 1 => Some(amount),
                2 => Some(amount * -1.0),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_val_out(&self) -> Option<f32> {
        self.ext_value_out
    }

    pub fn get_bang_signal(&self) -> bool {
        self.note_bang
    }

    pub fn get_toggable(&self) -> bool {
        self.toggable
    }
}

impl Default for Event {
    fn default() -> Self {
        Event {
            index: 0,
            name: "A No op Event".to_string(),
            mesg_in: Vec::new(),
            mesg_out: None,
            ext_value_out: None,
            mod_rule: 0,
            mod_amount: None,
            note_bang: false,
            toggable: false,
        }
    }
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut amount = "".to_string();
        let rule = match self.mod_rule {
            0 => "In->Out".to_string(),
            1 => {
                amount = "+".to_string();
                "(In+amount)->Out".to_string()
            }
            2 => {
                amount = "-".to_string();
                "(In-amount)->Out".to_string()
            }
            _ => "Unkwown Rule".to_string(),
        };
        write!(
            f,
            "Midi Event #{} {} : \n Wait for : {:?} (note_bang : {}), Send : {{ {:?} }} following rule : {} by amount : {}{:?} \n Sending to client : {:?}",
            self.index+1,
            self.name,
            self.mesg_in,
            self.note_bang,
            self.mesg_out,
            rule,
            amount,
            self.mod_amount,
            self.ext_value_out
        )
    }
}

#[derive(Clone)]
pub struct DeviceState {
    timestamp: [usize; 4],
    lcd_vec: Option<Vec<(u8, u8, String)>>,
    lcd_string: Option<String>,
    vpot: Vec<[u8; 3]>,
    faders: Vec<(u8, f32)>,
    chan_btns: Vec<(u8, u8, bool)>,
    fps: u64,
}

impl DeviceState {
    pub fn new(
        timestamp: [usize; 4],
        lcd_vec: Option<Vec<(u8, u8, String)>>, // (lcd_#, line_#, Message)
        lcd_string: Option<String>,
        vpot: Vec<[u8; 3]>,             // [vpot_idx, mode, value]
        faders: Vec<(u8, f32)>,         // [fader_num, pb_value]
        chan_btns: Vec<(u8, u8, bool)>, // (chan_#, btn_#, on/off)
        fps: u64,
    ) -> Result<DeviceState, String> {
        if let Some(lcd_vec) = &lcd_vec {
            if lcd_vec.len() > 14 {
                return Err("LCD length".to_string());
            }
        }

        for chan_btns in &chan_btns[..] {
            let channel = chan_btns.0;
            let btn_num = chan_btns.1;

            let note: u8 = btn_num * 8 + channel;

            if !(0x00..=0x1F).contains(&note) {
                return Err("Channel button not in range".to_string());
            }
        }

        Ok(Self {
            timestamp,
            lcd_vec,
            lcd_string,
            vpot,
            faders,
            chan_btns,
            fps,
        })
    }

    pub fn get_timestamp(&self) -> &[usize; 4] {
        &self.timestamp
    }

    pub fn get_lcd_vec(&self) -> &Option<Vec<(u8, u8, String)>> {
        &self.lcd_vec
    }

    pub fn get_lcd_string(&self) -> &Option<String> {
        &self.lcd_string
    }

    pub fn get_vpots(&self) -> &Vec<[u8; 3]> {
        &self.vpot
    }

    pub fn get_faders(&self) -> &Vec<(u8, f32)> {
        &self.faders
    }

    pub fn get_chan_btns(&self) -> &Vec<(u8, u8, bool)> {
        &self.chan_btns
    }

    pub fn get_fps(&self) -> &u64 {
        &self.fps
    }

    pub fn set_fps(&mut self, fps: u64) {
        self.fps = fps
    }

    pub fn set_timestamp(&mut self, hours: usize, minutes: usize, seconds: usize, frames: usize) {
        self.timestamp = [hours, minutes, seconds, frames]
    }
}

impl Default for DeviceState {
    fn default() -> Self {
        DeviceState {
            timestamp: [0; 4],
            lcd_vec: None,
            lcd_string: Some("This is a sample LCD string".to_string()),
            vpot: Vec::new(),
            faders: Vec::new(),
            chan_btns: Vec::new(),
            fps: 24,
        }
    }
}

impl std::fmt::Debug for DeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let frames = &self.timestamp[0];
        let seconds = &self.timestamp[1];
        let minutes = &self.timestamp[2];
        let hours = &self.timestamp[3];
        write!(f, "Initialize Device with values : \n Timestamp (fps : {}) : {:?}h {:?}m {:?}s {:?}f \n Lcd values : {:?} \n VPot : {:?} {:?} \n Faders : {:?} \n Channels Buttons : {:?}",
            self.fps,
            hours,
            minutes,
            seconds,
            frames,
            self.lcd_vec,
            self.lcd_string,
            self.vpot,
            self.faders,
            self.chan_btns
        )
    }
}

#[derive(Debug, Clone)]
pub struct MidiMesg {
    pub channel: u8,
    pub name: String,
    pub value: f32,
}

impl MidiMesg {
    pub fn new() -> Self {
        Self {
            channel: 0,
            name: "".to_string(),
            value: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct MidiProcess {
    pub debug: Option<MidiMesg>,
    pub to_send: TriggerResult,
}

pub type MidiResult = Result<MidiProcess, &'static str>;
