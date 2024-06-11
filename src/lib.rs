pub mod audio_out;
pub mod synth;
pub mod midi_scheduler;
pub mod player;

const SAMPLE_RATE: u32 = 8000;
const BIT_DEPTH: u16 = 8;
const BUFFER_SIZE: usize = 1024;