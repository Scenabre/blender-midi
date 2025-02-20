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
