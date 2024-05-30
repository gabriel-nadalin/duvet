use crate::{envelope::{Envelope, EnvelopeState}, oscillator::{Oscillator, Waveform}};

#[derive(Clone, Copy, Debug)]
pub struct Note {
    oscillator: Oscillator,
    envelope: Envelope,
}

impl Note {
    pub fn new(waveform: Waveform, frequency: f32, attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            oscillator: Oscillator::new(waveform, frequency),
            envelope: Envelope::new(attack, decay, sustain, release),
        }
    }

    pub fn from_env(waveform: Waveform, frequency: f32, envelope: Envelope) -> Self {
        Self {
            oscillator: Oscillator::new(waveform, frequency),
            envelope
        }
    }

    pub fn state(&self) -> EnvelopeState {
        self.envelope.state()
    }

    pub fn note_on(&mut self) {
        self.envelope.trigger();
    }

    pub fn note_off(&mut self) {
        self.envelope.release();
    }

    pub fn next_sample(&mut self) -> f32 {
        let amplitude = self.envelope.get_amplitude();
        amplitude * self.oscillator.next_sample()
    }
}