use crate::synth::{envelope::{Envelope, EnvelopeKind, EnvelopeState}, oscillator::{Oscillator, Waveform}, instrument::Instrument};

#[derive(Clone, Copy, Debug)]
pub struct Note {
    oscillator: Oscillator,
    lfo: Oscillator,
    envelope: Envelope,
    freq_envelope: Option<Envelope>,
    frequency: f32,
    lfo_amplitude: f32,
    noise: f32,
}

impl Note {
    // pub fn new(waveform: Waveform, frequency: f32, lfo_waveform: Waveform, lfo_frequency: f32, lfo_amplitude: f32, attack: f32, decay: f32, sustain: f32, release: f32, env_kind: EnvelopeKind, noise: f32) -> Self {
    //     Self {
    //         oscillator: Oscillator::new(waveform, frequency),
    //         lfo: Oscillator::new(lfo_waveform, lfo_frequency),
    //         envelope: Envelope::new(attack, decay, sustain, release, env_kind),
    //         frequency,
    //         lfo_amplitude,
    //         noise,
    //     }
    // }

    pub fn from_env(waveform: Waveform, frequency: f32, lfo_amplitude: f32, lfo: Oscillator, envelope: Envelope, freq_envelope: Option<Envelope>, noise: f32) -> Self {
        Self {
            oscillator: Oscillator::new(waveform, frequency),
            lfo,
            envelope,
            freq_envelope,
            frequency,
            lfo_amplitude,
            noise,
        }
    }

    pub fn state(&self) -> EnvelopeState {
        self.envelope.state()
    }

    pub fn note_on(&mut self) {
        self.envelope.trigger();

        if let Some(ref mut envelope) = self.freq_envelope {
            envelope.trigger();
        }
    }

    pub fn note_off(&mut self) {
        self.envelope.release();
        
        if let Some(ref mut envelope) = self.freq_envelope {
            envelope.release();
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let amplitude = self.envelope.get_amplitude();
        let lfo_value = self.lfo.next_sample();

        if let Some(ref mut envelope) = self.freq_envelope {
            self.oscillator.set_frequency(self.frequency * envelope.get_amplitude());
        }
        // self.oscillator.set_frequency(self.frequency * (1.0 + lfo_value * self.lfo_amplitude));

        let noise = 2. * rand::random::<f32>() - 1.;
        let sample = (1.0 - self.noise) * self.oscillator.next_sample() + self.noise * noise;
        amplitude * sample
    }
}