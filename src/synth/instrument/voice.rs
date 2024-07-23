use std::collections::HashMap;

use crate::{
    midi2freq,
    synth::{
        instrument::Instrument,
        effect::Effect,
        envelope::{Envelope, EnvelopeShape, EnvelopeState},
        note::Note,
        oscillator::{Oscillator, Waveform},
    }
};

pub struct Voice {
    waveform: Waveform,
    // oscillators: Vec<Oscillator>,        // TODO: substitute single waveform for this
    lfo: Oscillator,
    lfo_amplitude: f32,
    amp_envelope: Envelope,
    freq_envelope: Option<Envelope>,
    effects: Vec<Effect>,
    volume: f32,
    notes: HashMap<u32, Note>,              // Key is MIDI note number
}

impl Voice {
    pub fn new(waveform: Waveform, lfo: Oscillator, lfo_amplitude: f32, amp_envelope: Envelope, freq_envelope: Option<Envelope>, volume: f32, effects: Vec<Effect>) -> Self {
        Self {
            waveform,
            lfo,
            lfo_amplitude,
            amp_envelope,
            freq_envelope,
            effects,
            volume,
            notes: HashMap::new(),
        }
    }

    pub fn frequency_on(&mut self, frequency: f32) {
        let mut note = Note::from_env(self.waveform, frequency, self.lfo_amplitude, self.lfo, self.amp_envelope, self.freq_envelope, 0., self.effects.clone());
        note.note_on();
        self.notes.insert(frequency as u32, note);
    }

    pub fn frequency_off(&mut self, frequency: f32) {
        if let Some(note) = self.notes.get_mut(&(frequency as u32)) {
            note.note_off();
        }
    }

    pub fn lead_square(volume: f32) -> Self {
        let waveform = Waveform::Square;
        let envelope = Envelope::new(0.03, 0.1, 0.7, 0.6, EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        let effects = vec![];
        Self::new(waveform, lfo, 0.008, envelope, None, volume, effects)
    }

    pub fn lead_sine(volume: f32) -> Self {
        let waveform = Waveform::Sine;
        let envelope = Envelope::new(0.03, 0.1, 0.7, 0.6, EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        // let effects = vec![Effect::Gain(5.), Effect::Fangs(1.), Effect::HardClip(1.), Effect::Gain(5.), ];           // very cool wave shape
        // let effects = vec![Effect::Gain(3.), Effect::SoftCubic(1.), Effect::Gain(1.), ];                             // spooky ghosts
        let effects = vec![Effect::Gain(5.), ];
        Self::new(waveform, lfo, 0.01, envelope, None, volume, effects)
    }

    pub fn lead_sawtooth(volume: f32) -> Self {
        let waveform = Waveform::Sawtooth;
        let envelope = Envelope::new(0.03, 0.1, 0.7, 0.6, EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        let effects = vec![Effect:: Gain(4.), Effect::HardClip(1.)];
        Self::new(waveform, lfo, 0.005, envelope, None, volume, effects)
    }

    pub fn lead_triangle(volume: f32) -> Self {
        let waveform = Waveform::Triangle;
        let envelope = Envelope::new(0.03, 0.1, 0.7, 0.6, EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        let effects = vec![];
        Self::new(waveform, lfo, 0.005, envelope, None, volume, effects)
    }

    pub fn drum_kit(volume: f32) -> Self {
        let waveform = Waveform::Triangle;
        let envelope = Envelope::new(0., 0., 0., 0., EnvelopeShape::Exponential);
        let lfo = Oscillator::new(Waveform::Sine, 5.);
        let effects = vec![];
        Self::new(waveform, lfo, 0.005, envelope, None, volume, effects)
    }
}

impl Instrument for Voice {
    fn note_on(&mut self, midi_note: u8) {
        let frequency = midi2freq(midi_note);
        let mut note = Note::from_env(self.waveform, frequency, self.lfo_amplitude, self.lfo, self.amp_envelope, self.freq_envelope, 0., self.effects.clone());
        note.note_on();
        self.notes.insert(frequency as u32, note);
    }

    fn note_off(&mut self, midi_note: u8) {
        let frequency = midi2freq(midi_note);
        if let Some(note) = self.notes.get_mut(&(frequency as u32)) {
            note.note_off();
        }
    }

    fn next_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        self.notes.retain(|_, note| {
            let note_sample = note.next_sample();
            sample += note_sample;
            !matches!(note.state(), EnvelopeState::Idle)
        });
        sample * self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume
    }
}