use std::collections::HashMap;

use crate::synth::{envelope::{Envelope, EnvelopeShape, EnvelopeState}, note::Note, oscillator::{Oscillator, Waveform}, drum_machine::DrumMachine};

pub enum InstrumentKind {
    Melodic,
    Percussive,
}

pub enum InstrumentMode {
    Legato,
    Monophonic,
    Polyphonic,
}

pub struct Instrument {
    kind: InstrumentKind,
    waveform: Waveform,
    lfo: Oscillator,
    lfo_amplitude: f32,
    amp_envelope: Envelope,
    freq_envelope: Option<Envelope>,
    volume: f32,
    notes: HashMap<u8, Note>, // Key is MIDI note number
}

impl Instrument {
    pub fn new(kind: InstrumentKind, waveform: Waveform, lfo: Oscillator, lfo_amplitude: f32, amp_envelope: Envelope, freq_envelope: Option<Envelope>, volume: f32) -> Self {
        Self {
            kind,
            waveform,
            lfo,
            lfo_amplitude,
            amp_envelope,
            freq_envelope,
            volume,
            notes: HashMap::new(),
        }
    }

    pub fn note_on(&mut self, midi_note: u8) {
        let mut note = match self.kind {
            InstrumentKind::Melodic => {
                let frequency = midi2freq(midi_note);
                Note::from_env(self.waveform, frequency, self.lfo_amplitude, self.lfo, self.amp_envelope, self.freq_envelope, 0.)
            }
            InstrumentKind::Percussive => {
                match midi_note {
                    35 | 36 | 43 => DrumMachine::kick(),
                    38 | 40 | 45 | 47 => DrumMachine::snare(),
                    42 | 44 | 46 | 53 => DrumMachine::hihat(),
                    49 | 52 | 57 => DrumMachine::cymbal(),
                    _ => return
                }
            }
        };
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
        let envelope = Envelope::new(0.03, 0.1, 0.7, 0.6, EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.008, envelope, None, volume)
    }

    pub fn lead_sine(volume: f32) -> Self {
        let kind = InstrumentKind::Melodic;
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0.03, 0.1, 0.7, 0.6, EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.005, envelope, None, volume)
    }

    pub fn lead_sawtooth(volume: f32) -> Self {
        let kind = InstrumentKind::Melodic;
        let waveform = Waveform::Sawtooth;
        let envelope = Envelope::new(0.03, 0.1, 0.7, 0.6, EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.005, envelope, None, volume)
    }

    pub fn lead_triangle(volume: f32) -> Self {
        let kind = InstrumentKind::Melodic;
        let waveform = Waveform::Triangle;
        let envelope = Envelope::new(0.03, 0.1, 0.7, 0.6, EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.005, envelope, None, volume)
    }

    pub fn drum_kit(volume: f32) -> Self {
        let kind = InstrumentKind::Percussive;
        let waveform = Waveform::Triangle;
        let envelope = Envelope::new(0., 0., 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        Self::new(kind, waveform, lfo, 0.005, envelope, None, volume)
    }
}


fn midi2freq(midi_note: u8) -> f32 {
    const A4: f32 = 440.0; // Frequency of A4
    const A4_MIDI: u8 = 69; // MIDI note number of A4

    A4 * 2f32.powf((midi_note as f32 - A4_MIDI as f32) / 12.0)
}