use midly::MidiMessage;

use crate::{audio_out::{AudioMode, AudioOut}, instrument::Instrument, midi_scheduler::MidiScheduler, synth::Synth, SAMPLE_RATE};

pub enum PlayerKind {
    Keyboard,
    Midi(MidiScheduler),
}
pub struct Player {
    synth: Synth,
    kind: PlayerKind,
    out: AudioOut,
    time: f64,
}

impl Player {
    pub fn new_midi(f: &[u8], mode: AudioMode) -> Self {
        let mut synth = Synth::new();

        let voice = Instrument::lead_square(0.15);
        let bass = Instrument::lead_square(0.15);
        let guitar = Instrument::lead_triangle(0.15);
        let guitar2 = Instrument::lead_sawtooth(0.15);

        synth.add_instrument(0, voice);
        synth.add_instrument(1, bass);
        synth.add_instrument(2, guitar);
        synth.add_instrument(6, guitar2);
        
        let scheduler = MidiScheduler::new(f);
        let kind = PlayerKind::Midi(scheduler);
        let out = AudioOut::new(mode);

        Self {
            synth,
            kind,
            out,
            time: 0.,
        }
    }

    pub fn update(&mut self) {
        match self.kind {
            PlayerKind::Midi(_) => self.midi_update(),
            PlayerKind::Keyboard => todo!(),
        }
        self.time += 1. / SAMPLE_RATE as f64;
    }

    fn midi_update(&mut self) {
        if let PlayerKind::Midi(ref mut scheduler) = self.kind {
            if let Some((timestamp, channel, message)) = scheduler.current_event() {
                if timestamp <= self.time {
                    match message {
                        MidiMessage::NoteOn { key, vel } => {
                            if vel == 0 {
                                self.synth.note_off(channel, key.as_int());
                            }
                            else {
                                self.synth.note_on(channel, key.as_int());
                            }
                        }
                        MidiMessage::NoteOff { key, .. } => {
                            self.synth.note_off(channel, key.as_int());
                        }
                        _ => ()
                    }

                    scheduler.next_event();
                }
            }
            else {
                self.out.drain();
                panic!()
            }
        }

        let sample = self.synth.next_sample();
        self.out.send(bipolar2u8(sample));
    }
}

fn bipolar2u8(sample: f32) -> u8 {
    let sample = ((sample + 1.) / 2.) * 255.;
    sample.round() as u8
}