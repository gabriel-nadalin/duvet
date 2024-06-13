use std::{env, path::Path};

use duvet::{audio_out::{AudioMode, AudioOut}, synth::instrument::Instrument, player::Player};



fn main() {
    let mut drum_kit = Instrument::drum_kit(1.);
    let mut out = AudioOut::new(AudioMode::Play);

    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(&args[1]);

    let mut player = Player::new_midi(file_path, AudioMode::Play);
    // let mut player = Player::new_midi(include_bytes!("../../midi/duvet.mid"), AudioMode::Play);

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

