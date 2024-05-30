use std::fs::File;
use std::io::BufWriter;

use alsa::Direction;
use alsa::pcm::{Access, Format, HwParams, IO, PCM};
use alsa::ValueOr;
use hound::{self, WavWriter};

use crate::{BUFFER_SIZE, SAMPLE_RATE};

pub fn set_pcm_params(pcm: &alsa::PCM) {
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::U8).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
}

trait Writer {
    fn write(&mut self, buffer: &Vec<u8>);
    fn drain(&self);
}

struct PCMWriter {
    pcm: PCM,
}

impl PCMWriter {
    fn new() -> Self {
        let pcm = PCM::new("default", Direction::Playback, false).unwrap();
        set_pcm_params(&pcm);
        Self {
            pcm,
        }
    }
}

impl Writer for PCMWriter {
    fn write(&mut self, buffer: &Vec<u8>) {
        let io = self.pcm.io_u8().unwrap();
        io.writei(buffer).unwrap();
    }

    fn drain(&self) {
        self.pcm.drain().unwrap();
    }
}

struct WAVWriter {
    writer: WavWriter<BufWriter<File>>,
}

impl WAVWriter {
    fn new() -> Self {
        
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 8,
            sample_format: hound::SampleFormat::Int,
        };

        let writer = hound::WavWriter::create("output.wav", spec).unwrap();

        Self {
            writer,
        }
    }
}

impl Writer for WAVWriter {
    fn write(&mut self, buffer: &Vec<u8>) {
        for &sample in buffer {
            self.writer.write_sample((sample as i16 - 128) as i8).unwrap();
        }
    }

    fn drain(&self) {
    }
}

pub enum AudioMode {
    Play,
    Record,
}

pub struct AudioOut {
    writer: Box<dyn Writer>,
    buffer: Vec<u8>
}

impl AudioOut {

    pub fn new(mode: AudioMode) -> Self {
        let writer: Box<dyn Writer> = match mode {
            AudioMode::Play => Box::new(PCMWriter::new()),
            AudioMode::Record => Box::new(WAVWriter::new()),
        };

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

    pub fn drain(&self) {
        self.writer.drain();
    }
}