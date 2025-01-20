pub const MAX_MIDI_MSG_SIZE: usize = 16;

#[derive(Clone, Copy)]
pub struct RawMidi {
    /// The amount of time passed, in frames, relative to the start of the process cycle.
    pub delta_frames: u64, // stamp

    data: [u8; MAX_MIDI_MSG_SIZE],

    len: u8,
}

impl RawMidi {
    /// Create a new midi message from raw bytes.
    ///
    /// * `delta_frames` - The amount of time passed, in frames, relative to the start of the process cycle.
    /// * `data` - The raw bytes of the midi message.
    ///
    /// This returns an error if the length of `data` is greater than `MAX_MIDI_MSG_SIZE` (8).
    pub fn new(delta_frames: u64, data: &[u8]) -> Result<Self, usize> {
        if data.len() <= MAX_MIDI_MSG_SIZE {
            let mut cp_data = [0; MAX_MIDI_MSG_SIZE];
            cp_data[0..data.len()].copy_from_slice(data);

            Ok(Self {
                delta_frames,
                data: cp_data,
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

    /// The size of this MIDI message in bytes.
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }
}

impl std::fmt::Debug for RawMidi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Raw MIDI: {{ delta frames: {}, len: {}, data: {:?} }}",
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
            data: [0; MAX_MIDI_MSG_SIZE],
            len: 0,
        }
    }
}
