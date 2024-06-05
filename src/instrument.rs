use std::collections::HashMap;

use crate::{envelope::{Envelope, EnvelopeKind, EnvelopeState}, midi2freq, note::Note, oscillator::{Oscillator, Waveform}};

pub enum InstrumentKind {
    Melodic,
    Percussive,
}

pub struct Instrument {
    kind: InstrumentKind,
    waveform: Waveform,
    lfo: Oscillator,
    lfo_amplitude: f32,
    envelope: Envelope,
    volume: f32,
    notes: HashMap<u8, Note>, // Key is MIDI note number
}

impl Instrument {
    pub fn new(kind: InstrumentKind, waveform: Waveform, lfo: Oscillator, lfo_amplitude: f32, envelope: Envelope, volume: f32) -> Self {
        Self {
            kind,
            waveform,
            lfo,
            lfo_amplitude,
            envelope,
            volume,
            notes: HashMap::new(),
        }
    }

    pub fn note_on(&mut self, midi_note: u8) {
        let mut note;
        match self.kind {
            InstrumentKind::Melodic => {
                let frequency = midi2freq(midi_note);
                note = Note::from_env(self.waveform, frequency, self.lfo_amplitude, self.lfo, self.envelope, 0.);
            }
            InstrumentKind::Percussive => {
                match midi_note {
                    35 | 36 | 43 => note = Instrument::kick(),
                    40 | 45 | 47 => note = Instrument::snare(),
                    44 | 46 | 53 => note = Instrument::hihat(),
                    49 | 52 | 57 => note = Instrument::cymbal(),
                    _ => return
                }
            }
        }
        note.note_on();
        self.notes.insert(midi_note, note);
    }

    pub fn note_off(&mut self, midi_note: u8) {
        if let Some(note) = self.notes.get_mut(&midi_note) {
            note.note_off();
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        self.notes.retain(|_, note| {
            let note_sample = note.next_sample();
            sample += note_sample;
            !matches!(note.state(), EnvelopeState::Idle)
        });
        sample * self.volume
    }

    pub fn lead_square(volume: f32) -> Self {
        let kind = InstrumentKind::Melodic;
        let waveform = Waveform::Square;
        let envelope = Envelope::new(0.1, 0.1, 0.7, 0.6, EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.005, envelope, volume)
    }

    pub fn lead_sine(volume: f32) -> Self {
        let kind = InstrumentKind::Melodic;
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0.1, 0.1, 0.7, 0.6, EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.005, envelope, volume)
    }

    pub fn lead_sawtooth(volume: f32) -> Self {
        let kind = InstrumentKind::Melodic;
        let waveform = Waveform::Sawtooth;
        let envelope = Envelope::new(0.1, 0.1, 0.7, 0.6, EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.005, envelope, volume)
    }

    pub fn lead_triangle(volume: f32) -> Self {
        let kind = InstrumentKind::Melodic;
        let waveform = Waveform::Triangle;
        let envelope = Envelope::new(0.1, 0.1, 0.7, 0.6, EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.005, envelope, volume)
    }

    pub fn drum_kit(volume: f32) -> Self {
        let kind = InstrumentKind::Percussive;
        let waveform = Waveform::Triangle;
        let envelope = Envelope::new(0., 0., 0., 0., EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.005, envelope, volume)
    }

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
