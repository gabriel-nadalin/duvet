mod synth;

use synth::{Envelope};
use alsa::{pcm::{Access, Format, HwParams}, Direction, ValueOr, PCM};

const SAMPLE_RATE: u32 = 44100;

pub fn set_pcm_params(pcm: &alsa::PCM) {
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::U8).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
}

fn main() {
    let pcm = PCM::new("default", Direction::Playback, false).unwrap();
    set_pcm_params(&pcm);
    let io = pcm.io_u8().unwrap();
    let mut envelope = Envelope::new();
    let mut time = 0.;
    envelope.note_on(time);

    for _ in 0..SAMPLE_RATE * 3 {
        let sample = envelope.get_sample1(time);
        // dbg!(envelope.get_amplitude(time));
        time += 1./ SAMPLE_RATE as f64;
        io.writei(&[sample]).unwrap();
        // dbg!(sample, time);
    }

    // loop {
    //     let sample = 0;

    //     // do some synthy stuff

    //     io.writei(&[sample]).unwrap();
    // }

    pcm.drain().unwrap();
}
