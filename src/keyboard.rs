#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NoteState {
    Attack,
    Decay,
    Release,
    Sustain,
    Off,
}

const KEYBOARD_SIZE: usize = 32;

#[derive(Copy, Clone, Debug)]
pub struct Note {
    pub state: NoteState,
    pub duration: f64,
    pub release_time: f64,
    pub freq: f64,
}

impl Default for Note {
    fn default() -> Note {
        Self {
            state: NoteState::Off,
            duration: 0.0f64,
            release_time: 0.0f64,
            freq: 0.0f64,
        }
    }
}

pub struct Keyboard {
    pub notes: [Note; KEYBOARD_SIZE],
}

impl Default for Keyboard {
    fn default() -> Keyboard {
        Self {
            notes: [Note::default(); KEYBOARD_SIZE],
        }
    }
}
