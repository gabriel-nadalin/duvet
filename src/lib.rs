pub mod audio_out;
pub mod synth;
pub mod midi_scheduler;
pub mod player;

const SAMPLE_RATE: u32 = 48000;
const BIT_DEPTH: u16 = 8;
const BUFFER_SIZE: usize = 1024;

fn midi2freq(midi_note: u8) -> f32 {
    const A4: f32 = 440.0; // Frequency of A4
    const A4_MIDI: u8 = 69; // MIDI note number of A4

    A4 * 2f32.powf((midi_note as f32 - A4_MIDI as f32) / 12.0)
}

pub fn bipolar2u8(sample: f32) -> u8 {
    let sample = ((sample + 1.) / 2.) * 255.;
    sample.round() as u8
}