use std::{fs::File, io::Read, path::Path};

use midly::Smf;

pub struct MidiScheduler {
    events: Vec<(f64, u8, midly::MidiMessage)>, // (timestamp in seconds, channel, MIDI message)
    cursor: usize,
}

impl MidiScheduler {
    pub fn new(file_path: &Path) -> Self {

        // opening midi file
        let mut file = File::open(file_path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        // configuring midi reader
        let smf = Smf::parse(&buffer).unwrap();
        let mut tempo = 500_000;        // default midi tempo
        let ticks_per_beat = match smf.header.timing {
            midly::Timing::Metrical(value) => value.as_int(),
            _ => 480,
        };
        let mut events = Vec::new();

        // getting tempo
        for track in &smf.tracks {
            for event in track {
                if let midly::TrackEventKind::Meta(message) = &event.kind {
                    if let midly::MetaMessage::Tempo(t) = message {
                        tempo = t.as_int();
                    }
                }
            }
        }
        
        // reading midi file
        for track in &smf.tracks {
            let mut time = 0;
            for event in track {
                time += event.delta.as_int();
                if let midly::TrackEventKind::Midi { message, channel } = &event.kind {
                    let timestamp = time as f64 * (tempo as f64 / 1_000_000.0) / ticks_per_beat as f64;
                    events.push((timestamp, channel.as_int(), message.clone()));
                }
                // else if let midly::TrackEventKind::Meta(message) = &event.kind {
                //     if let midly::MetaMessage::Tempo(t) = message {
                //         tempo = t.as_int();
                //     }
                // }
            }
        }

        // sorting events by timestamp
        events.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        Self {
            events,
            cursor: 0,
        }
    }

    pub fn current_event(&mut self) -> Option<(f64, u8, midly::MidiMessage)> {
        if self.cursor >= self.events.len() {
            None
        }
        else {
            Some(self.events[self.cursor])
        }
    }

    pub fn next_event(&mut self) {
        self.cursor += 1;
    }
}