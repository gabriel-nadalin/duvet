use std::{collections::HashSet, io::Read, path::Path};

use midly::MidiMessage;
use termion::{async_stdin, event::Key, input::TermRead};

use crate::{audio_out::{AudioMode, AudioOut}, synth::instrument::Instrument, midi_scheduler::MidiScheduler, synth::Synth, SAMPLE_RATE};

pub struct MidiPlayer {
    scheduler: MidiScheduler,
}

impl MidiPlayer {
    pub fn new(file_path: &Path) -> Self {
        let scheduler = MidiScheduler::new(file_path);
        Self {
            scheduler,
        }
    }

    fn update(&mut self, synth: &mut Synth, time: f64) -> bool {
        if let Some((timestamp, channel, message)) = self.scheduler.current_event() {
            if timestamp <= time {
                match message {
                    MidiMessage::NoteOn { key, vel } => {
                        if vel == 0 {
                            synth.note_off(channel, key.as_int());
                        }
                        else {
                            synth.note_on(channel, key.as_int());
                        }
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        synth.note_off(channel, key.as_int());
                    }
                    _ => ()
                }

                self.scheduler.next_event();
            }
            true
        }
        else {
            false
        }
    }
}

pub struct KeyboardPlayer {
    keys_pressed: HashSet<Key>,
    stdin: termion::AsyncReader,
    current_channel: u8,
}

impl KeyboardPlayer {
    pub fn new() -> Self {
        let stdin = async_stdin();
        let keys_pressed = HashSet::new();

        Self {
            stdin,
            keys_pressed,
            current_channel: 0,
        }
    }

    pub fn update(&mut self, synth: &mut Synth, time: f64) -> bool {
        let keys: Vec<_> = self.stdin.by_ref().events().collect();
        for key in keys {
            println!("{:?}", key);
            // if let Ok(key) = key {
            //     match key {
            //         Key::Char('a') => self.handle_key_event(Key::Char('a'), 60, synth), // C4
            //         Key::Char('s') => self.handle_key_event(Key::Char('s'), 62, synth), // D4
            //         Key::Char('d') => self.handle_key_event(Key::Char('d'), 64, synth), // E4
            //         Key::Char('f') => self.handle_key_event(Key::Char('f'), 65, synth), // F4
            //         Key::Char('g') => self.handle_key_event(Key::Char('g'), 67, synth), // G4
            //         _ => (),
            //     }
            // }
        }

        // Handle key releases
        let mut keys_pressed = self.keys_pressed.clone();
        keys_pressed.retain(|&key| {
            match key {
                Key::Char('a') => self.handle_key_release(Key::Char('a'), 60, synth),
                Key::Char('s') => self.handle_key_release(Key::Char('s'), 62, synth),
                Key::Char('d') => self.handle_key_release(Key::Char('d'), 64, synth),
                Key::Char('f') => self.handle_key_release(Key::Char('f'), 65, synth),
                Key::Char('g') => self.handle_key_release(Key::Char('g'), 67, synth),
                Key::Char('q') => return false,
                _ => true,
            }
        });
        self.keys_pressed = keys_pressed;
        true
    }
    
    fn handle_key_event(&mut self, key: Key, note: u8, synth: &mut Synth) {
        println!("aqui!");
        if !self.keys_pressed.contains(&key) {
            synth.note_on(self.current_channel, note);
            self.keys_pressed.insert(key);
        }
    }

    fn handle_key_release(&mut self, key: Key, note: u8, synth: &mut Synth) -> bool {
        if !self.stdin.by_ref().keys().any(|k| matches!(k, Ok(key))) {
            synth.note_off(self.current_channel, note);
            return false;
        }
        true
    }
}

pub enum PlayerKind {
    Keyboard(KeyboardPlayer),
    Midi(MidiPlayer),
    Both(KeyboardPlayer, MidiPlayer),
}

pub struct Player {
    synth: Synth,
    kind: PlayerKind,
    out: AudioOut,
    time: f64,
}

impl Player {
    pub fn new(kind: PlayerKind, audio_mode: AudioMode) -> Self {
        let mut synth = Synth::new();

        let voice = Instrument::lead_square(0.1);
        let bass = Instrument::lead_square(0.1);
        let bass2 = Instrument::lead_sine(0.1);
        let guitar = Instrument::lead_sawtooth(0.1);
        let guitar2 = Instrument::lead_sawtooth(0.1);
        let violin = Instrument::lead_triangle(0.1);
        let bell = Instrument::lead_square(0.1);
        let sine = Instrument::lead_sine(0.1);
        let drums = Instrument::drum_kit(0.3);
        let voice2 = Instrument::lead_triangle(0.1);

        synth.add_instrument(0, voice);
        synth.add_instrument(1, bass);
        synth.add_instrument(4, bass2);
        synth.add_instrument(2, guitar);
        synth.add_instrument(6, guitar2);
        synth.add_instrument(5, violin);
        synth.add_instrument(3, bell);
        synth.add_instrument(7, sine);
        synth.add_instrument(9, drums);
        synth.add_instrument(11, voice2);

        let out = AudioOut::new(audio_mode);

        Self {
            synth,
            kind,
            out,
            time: 0.,
        }
    }

    pub fn new_midi(file_path: &Path, audio_mode: AudioMode) -> Self {
        let midi_player = MidiPlayer::new(file_path);
        Self::new(PlayerKind::Midi(midi_player), audio_mode)
    }

    pub fn new_keyboard(audio_mode: AudioMode) -> Self {
        let keyboard_player = KeyboardPlayer::new();
        Self::new(PlayerKind::Keyboard(keyboard_player), audio_mode)
    }

    pub fn update(&mut self) -> bool {
        let condition;
        match &mut self.kind {
            PlayerKind::Midi(midi_player) => {
                condition = midi_player.update(&mut self.synth, self.time);
            }
            PlayerKind::Keyboard(keyboard_player) => {
                condition = keyboard_player.update(&mut self.synth, self.time);
            }
            PlayerKind::Both(keyboard_player, midi_player) => {
                midi_player.update(&mut self.synth, self.time);
                condition = keyboard_player.update(&mut self.synth, self.time);
            }
        }

        let sample = self.synth.next_sample();
        self.out.send(bipolar2u8(sample));
        self.time += 1. / SAMPLE_RATE as f64;
        condition
    }

    pub fn drain(&mut self) {
        self.out.drain()
    }
}

pub fn bipolar2u8(sample: f32) -> u8 {
    let sample = ((sample + 1.) / 2.) * 255.;
    sample.round() as u8
}