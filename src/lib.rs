// author: Rob Saunders <hello@robsaunders.io>

#[macro_use]
extern crate vst;
extern crate arrayvec;

use self::arrayvec::ArrayVec;
use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{CanDo, Category, Info, Plugin};

use std::f64::consts::PI;

const CHANNELS: usize = 64;

/// Convert the midi note's pitch into the equivalent frequency.
///
/// This function assumes A4 is 440hz.
fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    ((f64::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

struct Channel {
    key: u8,
    duration: f64,
    pressed: bool,
}

struct SineSynth {
    sample_rate: f64,
    time: f64,
    channels: ArrayVec<[Channel; CHANNELS]>,
}

impl SineSynth {
    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    /// Process an incoming midi event.
    ///
    /// The midi data is split up like so:
    ///
    /// `data[0]`: Contains the status and the channel. Source: [source]
    /// `data[1]`: Contains the supplemental data for the message - so, if this was a NoteOn then
    ///            this would contain the note.
    /// `data[2]`: Further supplemental data. Would be velocity in the case of a NoteOn message.
    ///
    /// [source]: http://www.midimountain.com/midi/midi_status.htm
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8) {
        self.channels.push(Channel {
            key: note,
            duration: 0.0f64,
            pressed: true,
        })
    }

    fn note_off(&mut self, note: u8) {
        for mut chan in self.channels.iter_mut() {
            if chan.key == note {
                chan.pressed = false;
                break;
            }
        }
    }
}

pub const TAU: f64 = PI * 2.0;

impl Default for SineSynth {
    fn default() -> SineSynth {
        SineSynth {
            sample_rate: 44100.0,
            time: 0.0,
            channels: ArrayVec::new(),
        }
    }
}

impl Plugin for SineSynth {
    fn get_info(&self) -> Info {
        Info {
            name: "SineSynth".to_string(),
            vendor: "DeathDisco".to_string(),
            unique_id: 6667,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            ..Info::default()
        }
    }

    // Supresses warning about match statment only having one arm
    #[allow(unknown_lints)]
    #[allow(unused_variables)]
    #[allow(clippy::single_match)]
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                // More events can be handled here.
                _ => (),
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();
        let (_, outputs) = buffer.split();
        let output_count = outputs.len();
        let per_sample = self.time_per_sample();

        self.channels.retain(|x| x.pressed == true); // Retain only pressed channels, remove others

        for sample_idx in 0..samples {
            let time = self.time;
            let mut output_sample = 0.0;

            for mut channel in self.channels.iter_mut() {
                output_sample +=
                    ((time * midi_pitch_to_freq(channel.key) * TAU).sin() * 0.1) as f32;

                channel.duration += per_sample;
            }
            self.time += per_sample;

            for buf_idx in 0..output_count {
                let buff = outputs.get_mut(buf_idx);
                buff[sample_idx] = output_sample;
            }
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(SineSynth);

#[cfg(test)]
mod tests {
    use crate::midi_pitch_to_freq;

    #[test]
    fn test_midi_pitch_to_freq() {
        for i in 0..127 {
            // expect no panics
            midi_pitch_to_freq(i);
        }
    }
}
