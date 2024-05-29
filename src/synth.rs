use crate::instrument::Instrument;

pub struct Synth {
    pub instruments: Vec<Instrument>,
}

impl Synth {
    pub fn new() -> Self {
        Synth {
            instruments: Vec::new(),
        }
    }

    pub fn add_instrument(&mut self, instrument: Instrument) {
        self.instruments.push(instrument);
    }

    pub fn next_sample(&mut self) -> f32 {
        self.instruments.iter_mut().map(|instr| instr.next_sample()).sum()
    }
}