use alsa::Direction;
use alsa::pcm::{Access, Format, HwParams, IO, PCM};
use alsa::ValueOr;
use crate::{BUFFER_SIZE, SAMPLE_RATE};

pub fn set_pcm_params(pcm: &alsa::PCM) {
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::U8).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
}

pub enum AudioMode {
    Play,
    Write,
}

pub struct AudioOut {
    mode: AudioMode,
    pcm: PCM,
    buffer: Vec<u8>
}

impl AudioOut {

    pub fn new(mode: AudioMode) -> Self {
        let pcm = PCM::new("default", Direction::Playback, false).unwrap();
        set_pcm_params(&pcm);
        let buffer = vec![];
        Self {
            mode,
            pcm,
            buffer,
        }
    }

    pub fn audio_out(&mut self, sample: u8) {
        self.buffer.push(sample);
        if self.buffer.len() >= BUFFER_SIZE {
            match self.mode {
                AudioMode::Play => {
                    let io = self.pcm.io_u8().unwrap();
                    io.writei(&self.buffer).unwrap();
                }
                AudioMode::Write => {
                    //TODO
                }
            }
            self.buffer.clear();
        }
    }

    pub fn drain(&mut self) {
        self.pcm.drain().unwrap();
    }
}