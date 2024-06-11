use crate::synth::{envelope::{Envelope, EnvelopeKind}, note::Note, oscillator::{Oscillator, Waveform}};

pub struct DrumMachine {

}

impl DrumMachine {
    pub fn kick() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0.01, 0.4, 0.8, 0.4, EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 80., 0.005, lfo, envelope, 0.1)
    }

    pub fn snare() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0.01, 0.4, 0.8, 0.4, EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 140., 0.005, lfo, envelope, 0.1)
    }

    pub fn hihat() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0.01, 0.2, 0.8, 0.2, EnvelopeKind::Linear);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 2000., 0.005, lfo, envelope, 1.)
    }

    pub fn cymbal() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0.03, 2., 0.8, 2., EnvelopeKind::Linear);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 2000., 0.005, lfo, envelope, 1.)
    }
}