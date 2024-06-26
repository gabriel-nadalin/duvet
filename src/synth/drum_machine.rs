use crate::synth::{envelope::{Envelope, EnvelopeShape}, note::Note, oscillator::{Oscillator, Waveform}};

pub struct DrumMachine {

}

impl DrumMachine {
    pub fn kick() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 0.5, 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        let mut note = Note::from_env(waveform, 90., 0.005, lfo, envelope, Some(envelope), 0.08);
        note.set_volume(4.);
        note
    }

    pub fn snare() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 0.4, 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        let mut note = Note::from_env(waveform, 140., 0.005, lfo, envelope, Some(envelope), 0.5);
        note.set_volume(2.);
        note
    }

    pub fn hihat() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 0.1, 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 2000., 0.005, lfo, envelope, Some(envelope), 1.)
    }

    pub fn cymbal() -> Note {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0., 2., 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Note::from_env(waveform, 2000., 0.005, lfo, envelope, Some(envelope), 1.)
    }
}