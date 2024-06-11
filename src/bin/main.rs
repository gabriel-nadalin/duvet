use std::{io::stdout, thread, time::Duration};

use duvet::{audio_out::{AudioMode, AudioOut}, synth::instrument::Instrument, player::Player};
use termion::raw::IntoRawMode;



fn main() {
    let mut drum_kit = Instrument::drum_kit(1.);
    let mut out = AudioOut::new(AudioMode::Play);

    let mut player = Player::new_midi(include_bytes!("../../midi/duvet.mid"), AudioMode::Play);
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

