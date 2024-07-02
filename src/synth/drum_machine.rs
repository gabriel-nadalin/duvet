use crate::synth::{effect:: Effect, envelope::{Envelope, EnvelopeShape}, note::Note, oscillator::{Oscillator, Waveform}};

pub struct DrumMachine {

}

impl DrumMachine {
    pub fn kick() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 0.5, 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        let effects = vec![Effect::Gain(3.5)];
        Note::from_env(waveform, 90., 0.005, lfo, envelope, Some(envelope), 0.05, effects)
    }

    pub fn snare() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 0.5, 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        let effects = vec![Effect::Gain(2.)];
        Note::from_env(waveform, 220., 0.005, lfo, envelope, Some(envelope), 0.4, effects)
    }

    pub fn hihat() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 0.1, 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        let effects = vec![];
        Note::from_env(waveform, 2000., 0.005, lfo, envelope, Some(envelope), 1., effects)
    }

    pub fn cymbal() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 2., 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        let effects = vec![];
        Note::from_env(waveform, 2000., 0.005, lfo, envelope, Some(envelope), 1., effects)
    }
}