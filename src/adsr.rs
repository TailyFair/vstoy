use crate::keyboard::{Note, NoteState};

pub struct ADSR {
    pub attack: f64,
    pub decay: f64,
    pub sustain: f64,
    pub release: f64,
}

impl Default for ADSR {
    fn default() -> ADSR {
        Self {
            attack: 0.0f64,
            decay: 0.0f64,
            sustain: 1.0f64,
            release: 0.0f64,
        }
    }
}

impl ADSR {
    pub fn new(attack: f64, decay: f64, sustain: f64, release: f64) -> ADSR {
        Self {
            attack,
            decay,
            sustain,
            release,
        }
    }

    pub fn sample(&self, note: &mut Note, value: f64) -> f64 {
        let mut mix = 0.0f64;

        match note.state {
            NoteState::Attack => {
                if note.duration < self.attack {
                    mix += value * (note.duration / self.attack);
                } else {
                    note.state = NoteState::Decay;
                    mix += value;
                }
            }
            NoteState::Decay => {
                // TODO Implement Decay
                note.state = NoteState::Sustain;
                mix += value;
            }
            NoteState::Sustain => {
                mix += value * self.sustain;
            }
            NoteState::Release => {
                if (note.duration - note.release_time) < self.release {
                    mix += value
                        * self.sustain
                        * (1.0 - ((note.duration - note.release_time) / self.release))
                } else {
                    note.state = NoteState::Off;
                }
            }

            NoteState::Off => (),
        }

        return mix;
    }
}
