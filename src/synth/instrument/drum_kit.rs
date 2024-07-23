use std::collections::HashMap;

use crate::synth::instrument::{Instrument, voice::Voice};

pub struct Drum {
    voice: Voice,
    frequency: f32,
}

impl Drum {
    pub fn note_on(&mut self) {
        self.voice.frequency_on(self.frequency);
    }

    pub fn note_off(&mut self) {
        self.voice.frequency_off(self.frequency);
    }

    pub fn next_sample(&mut self) -> f32 {
        self.voice.next_sample()
    }
}

pub struct DrumKit {
    parts: HashMap<u8, Drum>,            // key is midi code for drum part
    volume: f32,
}

impl DrumKit {
    pub fn new(volume: f32) -> Self {
        Self {
            parts: HashMap::new(),
            volume,
        }
    }

    pub fn add_part(&mut self, part: Drum, id: u8) {
        self.parts.insert(id, part);
    }
}

impl Instrument for DrumKit {
    fn note_on(&mut self, midi_note: u8) {
        if let Some(part) = self.parts.get_mut(&midi_note) {
            part.note_on();
        }
    }

    fn note_off(&mut self, midi_note: u8) {
        if let Some(part) = self.parts.get_mut(&midi_note) {
            part.note_off();
        }
    }

    fn next_sample(&mut self) -> f32 {
        let sample: f32 = self.parts.values_mut().map(|part| part.next_sample()).sum();
        sample * self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume
    }
}