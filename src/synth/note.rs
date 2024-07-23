use crate::synth::{
    effect:: Effect,
    envelope::{Envelope, EnvelopeState},
    oscillator::{Oscillator, Waveform}
};

pub struct Note {
    oscillator: Oscillator,
    lfo: Oscillator,
    lfo_amplitude: f32,
    amp_envelope: Envelope,
    freq_envelope: Option<Envelope>,
    effects: Vec<Effect>,
    frequency: f32,
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

    pub fn from_env(waveform: Waveform, frequency: f32, lfo_amplitude: f32, lfo: Oscillator, amp_envelope: Envelope, freq_envelope: Option<Envelope>, noise: f32, effects: Vec<Effect>) -> Self {
        Self {
            oscillator: Oscillator::new(waveform, frequency),
            lfo,
            lfo_amplitude,
            amp_envelope,
            freq_envelope,
            frequency,
            effects,
            noise,
        }
    }

    pub fn state(&self) -> EnvelopeState {
        self.amp_envelope.state()
    }

    pub fn note_on(&mut self) {
        self.amp_envelope.trigger();

        if let Some(ref mut envelope) = self.freq_envelope {
            envelope.trigger();
        }
    }

    pub fn note_off(&mut self) {
        self.amp_envelope.release();
        
        if let Some(ref mut envelope) = self.freq_envelope {
            envelope.release();
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let amplitude = self.amp_envelope.get_level();

        let lfo_value = self.lfo.next_sample();
        
        let mut frequency = self.frequency;

        if let Some(ref mut envelope) = self.freq_envelope {
            frequency *= envelope.get_level();
        }
        
        frequency *= 1.0 + lfo_value * self.lfo_amplitude;
        self.oscillator.set_frequency(frequency);
        
        let noise = 2. * rand::random::<f32>() - 1.;
        let mut sample = (1.0 - self.noise) * self.oscillator.next_sample() + self.noise * noise;

        // apply effects
        for effect in self.effects.clone() {
            sample = effect.apply(sample);
        }

        sample * amplitude
    }
}