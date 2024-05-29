use crate::oscillator::{Oscillator, Waveform};
use crate::envelope::Envelope;

pub struct Synth {
    oscillator: Oscillator,
    envelope: Envelope,
}

impl Synth {
    pub fn new(oscillator: Oscillator, envelope: Envelope) -> Self {
        Self { oscillator, envelope }
    }

    pub fn note_on(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
        self.envelope.trigger();
    }

    pub fn note_off(&mut self) {
        self.envelope.release();
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = self.oscillator.next_sample();
        sample * self.envelope.next_sample()
    }
}
