use crate::{envelope::{Envelope, EnvelopeKind, EnvelopeState}, oscillator::{Oscillator, Waveform}};

#[derive(Clone, Copy, Debug)]
pub struct Note {
    oscillator: Oscillator,
    lfo: Oscillator,
    envelope: Envelope,
}

impl Note {
    pub fn new(waveform: Waveform, frequency: f32, lfo_waveform: Waveform, lfo_frequency: f32, attack: f32, decay: f32, sustain: f32, release: f32, env_kind: EnvelopeKind) -> Self {
        Self {
            oscillator: Oscillator::new(waveform, frequency),
            lfo: Oscillator::new(lfo_waveform, lfo_frequency),
            envelope: Envelope::new(attack, decay, sustain, release, env_kind),
        }
    }

    pub fn from_env(waveform: Waveform, frequency: f32, lfo: Oscillator, envelope: Envelope) -> Self {
        Self {
            oscillator: Oscillator::new(waveform, frequency),
            lfo,
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
        let lfo_value = self.lfo.next_sample();
        // self.oscillator.set_frequency(self.oscillator.frequency() * (1.0 + lfo_value * 0.1));
        amplitude * self.oscillator.next_sample()
    }
}