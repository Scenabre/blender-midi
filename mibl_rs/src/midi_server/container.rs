pub const MAX_MIDI_MSG_SIZE: usize = 16;

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

    ///Clone data
    pub fn data_clone(&self) -> Vec<u8> {
        self.data.clone()
    }

    ///Clone delta_frames
    pub fn delta_frames_clone(&self) -> u64 {
        self.delta_frames.clone()
    }

    /// The size of this MIDI message in bytes.
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    /// Setter
    pub fn set(&mut self, delta_frames: u64, data: &[u8]) -> Result<(), usize> {
        if data.len() <= MAX_MIDI_MSG_SIZE {
            self.delta_frames = delta_frames;
            self.data = data.to_vec();
            self.len = data.len() as u8;

            Ok(())
        } else {
            Err(data.len())
        }
    }
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
    index: u8,
    name: String,
    mesg_in: Vec<u8>,
    cmd_out: Option<u8>,
    value_out: Option<Vec<u8>>,
    mod_rule: u8,            // 0 in->out, 1 in+x = out, 2 in-x = out,
    mod_amount: Option<f32>, // Increase/Descrease by
}

impl Event {
    pub fn new(
        index: u8,
        name: String,
        mesg_in: Vec<u8>,
        cmd_out: Option<u8>,
        value_out: Option<Vec<u8>>,
        mod_rule: u8,
        mod_amount: Option<f32>,
    ) -> Result<Event, String> {
        if mod_rule <= 2 {
            Ok(Self {
                index,
                name,
                mesg_in,
                cmd_out,
                value_out,
                mod_rule,
                mod_amount,
            })
        } else {
            Err("mod_rule must be one of the following value : 0,1,2".to_string())
        }
    }

    pub fn get_index(&self) -> &u8 {
        &self.index
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_mesg_in(&self) -> &Vec<u8> {
        &self.mesg_in
    }

    pub fn get_mesg_data(&self) -> Option<Vec<u8>> {
        match self.cmd_out {
            Some(cmd) => {
                let mut data = vec![cmd];
                match &self.value_out {
                    Some(value) => data.extend(value),
                    None => (),
                }
                Some(data)
            }
            None => None,
        }
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
}

impl Default for Event {
    fn default() -> Self {
        Event {
            index: 0,
            name: "A No op Event".to_string(),
            mesg_in: Vec::new(),
            cmd_out: None,
            value_out: None,
            mod_rule: 0,
            mod_amount: None,
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
            "Midi Event #{} {} : \n Wait for : {:?}, Send : {{ {:?} {:?} }} following rule : {} by amount : {}{:?}",
            self.index+1,
            self.name,
            self.mesg_in,
            self.cmd_out,
            self.value_out,
            rule,
            amount,
            self.mod_amount,
        )
    }
}
