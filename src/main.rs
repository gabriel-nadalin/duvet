mod oscillator;
mod audio_out;
mod envelope;
mod note;
mod instrument;
mod synth;
mod midi_scheduler;
mod player;

use std::{io::stdout, thread, time::Duration};

use audio_out::{AudioMode, AudioOut};
use instrument::Instrument;
use player::{bipolar2u8, Player};
use synth::Synth;
use termion::raw::IntoRawMode;

const SAMPLE_RATE: u32 = 8000;
const BIT_DEPTH: u16 = 8;
const BUFFER_SIZE: usize = 1024;



fn main() {
    let mut drum_kit = Instrument::drum_kit(1.);
    let mut out = AudioOut::new(AudioMode::Play);

    let mut player = Player::new_midi(include_bytes!("../midi/duvet.mid"), AudioMode::Play);
    // let mut player = Player::new_keyboard(AudioMode::Play);

    while player.update() {
        // thread::sleep(Duration::from_millis(50));
    }
    player.drain();

    // drum_kit.note_on(57);
    // loop {
    //     out.send(bipolar2u8(drum_kit.next_sample()));
    // }
    
}



fn midi2freq(midi_note: u8) -> f32 {
    const A4: f32 = 440.0; // Frequency of A4
    const A4_MIDI: u8 = 69; // MIDI note number of A4

    A4 * 2f32.powf((midi_note as f32 - A4_MIDI as f32) / 12.0)
}