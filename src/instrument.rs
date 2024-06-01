use std::collections::HashMap;

use crate::{envelope::{Envelope, EnvelopeKind, EnvelopeState}, midi2freq, note::Note, oscillator::{Oscillator, Waveform}};

pub struct Instrument {
    waveform: Waveform,
    lfo: Oscillator,
    lfo_amplitude: f32,
    envelope: Envelope,
    volume: f32,
    notes: HashMap<u8, Note>, // Key is MIDI note number
}

impl Instrument {
    pub fn new(waveform: Waveform, lfo: Oscillator, lfo_amplitude: f32, envelope: Envelope, volume: f32) -> Self {
        Self {
            waveform,
            lfo,
            lfo_amplitude,
            envelope,
            volume,
            notes: HashMap::new(),
        }
    }

    pub fn note_on(&mut self, midi_note: u8) {
        let frequency = midi2freq(midi_note);
        let mut note = Note::from_env(self.waveform, frequency, self.lfo_amplitude, self.lfo, self.envelope);
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
        let waveform = Waveform::Square;
        let envelope = Envelope::new(0.1, 0.05, 0.7, 0.2, EnvelopeKind::Linear);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(waveform, lfo, 0.005, envelope, volume)
    }

    pub fn lead_sine(volume: f32) -> Self {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0.1, 0.05, 0.7, 0.2, EnvelopeKind::Linear);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(waveform, lfo, 0.005, envelope, volume)
    }

    pub fn lead_sawtooth(volume: f32) -> Self {
        let waveform = Waveform::Sawtooth;
        let envelope = Envelope::new(0.1, 0.05, 0.7, 0.2, EnvelopeKind::Linear);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(waveform, lfo, 0.005, envelope, volume)
    }

    pub fn lead_triangle(volume: f32) -> Self {
        let waveform = Waveform::Triangle;
        let envelope = Envelope::new(0.1, 0.05, 0.7, 0.2, EnvelopeKind::Linear);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(waveform, lfo, 0.005, envelope, volume)
    }

    pub fn kick(volume: f32) -> Self {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0.01, 0.3, 0.0, 0.0, EnvelopeKind::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 1.);
        Self::new(waveform, lfo, 0.005, envelope, volume)
    }
}
