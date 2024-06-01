mod oscillator;
mod audio_out;
mod envelope;
mod note;
mod instrument;
mod synth;

use audio_out::{AudioMode, AudioOut};
use envelope::Envelope;
use instrument::Instrument;
use midly::{MidiMessage, Smf, TrackEventKind};
use num_traits::pow;
use oscillator::{Oscillator, Waveform};
use synth::Synth;

const SAMPLE_RATE: u32 = 8000;
const BUFFER_SIZE: usize = 1024;

fn bipolar2u8(sample: f32) -> u8 {
    let sample = ((sample + 1.) / 2.) * 255.;
    sample.round() as u8
}

fn main() {

    let mut audio_out = AudioOut::new(AudioMode::Play);

    // let mut osc = Oscillator::new(Waveform::Sine, 80.);
    // let mut env = Envelope::new(0.01, 0.1, 1., 0.1);
    let mut synth = Synth::new();
    let voice = Instrument::lead_square(0.5);
    synth.add_instrument(voice);
    
    let tempo = 500_000;
    let ticks_per_beat = 384;
    let mut cursor = 0;
    
    let mut smf = Smf::parse(include_bytes!("../duvet.mid")).unwrap();
    let track = &mut smf.tracks[0];
    let mut event = track[cursor];
    let mut counter = (delta2us(event.delta.as_int(), tempo, ticks_per_beat) as u64 * SAMPLE_RATE as u64 / pow(10., 6) as u64) as u32;
    
    // synth.instruments[0].note_on(69);
    
    loop {
        // cursor += 1;
        // if cursor == SAMPLE_RATE as usize * 3 {
        //     synth.instruments[0].note_off(69);
        //     // synth.instruments[0].note_on(72);
        // }
        // if cursor == SAMPLE_RATE as usize * 6 {
        //     break
        // }
        if counter == 0 {
            cursor += 1;
            if cursor >= track.len() { break }
            match event.kind {
                TrackEventKind::Midi { channel, message } => {
                    match message {
                        MidiMessage::NoteOn { key, vel } => {
                            // println!("{}", midi2freq(key.as_int()));
                            synth.instruments[0].note_on(key.as_int())
                        }
                        MidiMessage::NoteOff { key, vel } => {
                            synth.instruments[0].note_off(key.as_int());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            event = track[cursor];
            counter = (delta2us(event.delta.as_int(), tempo, ticks_per_beat) as u64 * SAMPLE_RATE as u64 / pow(10., 6) as u64) as u32;
            // println!("{counter}");
        }
        else {
            counter -= 1;
        }
        let sample = bipolar2u8(synth.next_sample());
        audio_out.send(sample);
    }
    audio_out.drain();
}

fn delta2us(delta_ticks: u32, tempo: u32, ticks_per_beat: u32) -> u32 {
    (tempo as u64 * delta_ticks as u64 / ticks_per_beat as u64) as u32
}

fn midi2freq(midi_note: u8) -> f32 {
    const A4: f32 = 440.0; // Frequency of A4
    const A4_MIDI: u8 = 69; // MIDI note number of A4

    A4 * 2f32.powf((midi_note as f32 - A4_MIDI as f32) / 12.0)
}