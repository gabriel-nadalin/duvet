use crate::synth::{envelope::{Envelope, EnvelopeKind}, note::Note, oscillator::{Oscillator, Waveform}};

pub struct DrumMachine {

}

impl DrumMachine {
    pub fn kick() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 0.4, 0., 0., EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 80., 0.005, lfo, envelope, Some(envelope), 0.2)
    }

    pub fn snare() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 0.4, 0., 0., EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 140., 0.005, lfo, envelope, Some(envelope), 0.5)
    }

    pub fn hihat() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 0.05, 0., 0., EnvelopeKind::Linear);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 2000., 0.005, lfo, envelope, Some(envelope), 1.)
    }

    pub fn cymbal() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 2., 0., 0., EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 2000., 0.005, lfo, envelope, Some(envelope), 1.)
    }
}