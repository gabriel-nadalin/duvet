use std::fs::File;
use std::io::BufWriter;

use alsa::Direction;
use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::ValueOr;
use hound::{self, WavWriter};

use crate::{BIT_DEPTH, BUFFER_SIZE, SAMPLE_RATE};

fn set_pcm_params(pcm: &alsa::PCM) {
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::U8).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
}

pub enum Writer {
    PCM(PCM),
    WAV(WavWriter<BufWriter<File>>),
}

impl Writer {
    fn new(mode: AudioMode) -> Self {
        match mode {
            AudioMode::Play => {
                let pcm = PCM::new("default", Direction::Playback, false).unwrap();
                set_pcm_params(&pcm);

                Self::PCM(pcm)
            }
            AudioMode::Record(file_name) => {
                let spec = hound::WavSpec {
                    channels: 1,
                    sample_rate: SAMPLE_RATE,
                    bits_per_sample: BIT_DEPTH,
                    sample_format: hound::SampleFormat::Int,
                };
                let file_name = "wav/".to_string() + &file_name + ".wav";
                let writer = hound::WavWriter::create(file_name, spec).unwrap();
        
                Self::WAV(writer)
            }
        }
    }

    fn write(&mut self, buffer: &Vec<u8>) {
        match self {
            Self::PCM(pcm) => {
                let io = pcm.io_u8().unwrap();
                io.writei(buffer).unwrap();
            }
            Self::WAV(writer) => {
                for &sample in buffer {
                    writer.write_sample((sample as i16 - 128) as i8).unwrap();
                }
            }
        }
    }

    fn drain(&mut self) {
        match self {
            Self::PCM(pcm) => pcm.drain().unwrap(),
            Self::WAV(_writer) => {}
        }
    }
}

pub enum AudioMode {
    Play,
    Record(String),
}

pub struct AudioOut {
    writer: Writer,
    buffer: Vec<u8>
}

impl AudioOut {

    pub fn new(mode: AudioMode) -> Self {
        let writer = Writer::new(mode);
        let buffer = vec![];

        Self {
            writer,
            buffer,
        }
    }

    pub fn send(&mut self, sample: u8) {
        self.buffer.push(sample);
        if self.buffer.len() >= BUFFER_SIZE {
            self.writer.write(&self.buffer);
            self.buffer.clear();
        }
    }

    pub fn drain(&mut self) {
        self.writer.drain();
    }
}