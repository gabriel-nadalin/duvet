pub mod drum_kit;
pub mod voice;

pub trait Instrument {
    fn note_on(&mut self, midi_note: u8);
    fn note_off(&mut self, midi_note: u8);
    fn next_sample(&mut self) -> f32;
    fn set_volume(&mut self, volume: f32);
}

pub enum InstrumentKind {
    Melodic,
    Percussive,
}

pub enum InstrumentMode {
    Legato,
    Monophonic,
    Polyphonic,
}