mod oscillator;
mod audio_out;
mod envelope;
mod note;
mod instrument;
mod synth;
mod midi_scheduler;
mod player;

use audio_out::{AudioMode, AudioOut};
use instrument::Instrument;
use player::Player;
use synth::Synth;

const SAMPLE_RATE: u32 = 8000;
const BUFFER_SIZE: usize = 1024;



fn main() {
    let mut synth = Synth::new();
    let voice = Instrument::lead_sawtooth(0.3);
    let bass = Instrument::lead_square(0.3);
    let guitar = Instrument::lead_triangle(0.3);

    synth.add_instrument(0, voice);
    synth.add_instrument(1, bass);
    synth.add_instrument(2, guitar);

    let mut player = Player::new_midi(include_bytes!("../duvet.mid"), AudioMode::Record);
    
    // synth.instruments[0].note_on(69);
    
    loop {
        player.update();
    }
}



fn midi2freq(midi_note: u8) -> f32 {
    const A4: f32 = 440.0; // Frequency of A4
    const A4_MIDI: u8 = 69; // MIDI note number of A4

    A4 * 2f32.powf((midi_note as f32 - A4_MIDI as f32) / 12.0)
}