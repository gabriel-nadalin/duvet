pub mod oscillator;
pub mod envelope;
pub mod instrument;
pub mod drum_machine;
pub mod note;

use std::collections::HashMap;

use instrument::Instrument;

pub struct Synth {
    instruments: HashMap<u8, Instrument>, // Key is instrument's channel number
}

impl Synth {
    pub fn new() -> Self {
        Synth {
            instruments: HashMap::new(),
        }
    }

    pub fn add_instrument(&mut self, channel: u8, instrument: Instrument) {
        self.instruments.insert(channel, instrument);
    }

    pub fn note_on(&mut self, channel: u8, midi_note: u8) {
        if let Some(instrument) = self.instruments.get_mut(&channel) {
            instrument.note_on(midi_note);
        }
    }

    pub fn note_off(&mut self, channel: u8, midi_note: u8) {
        if let Some(instrument) = self.instruments.get_mut(&channel) {
            instrument.note_off(midi_note);
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.instruments.values_mut().map(|instr| instr.next_sample()).sum()
    }
}