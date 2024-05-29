use crate::SAMPLE_RATE;

pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    state: EnvelopeState,
    level: f32,
    time: f32,
}

enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            state: EnvelopeState::Idle,
            level: 0.0,
            time: 0.0,
        }
    }

    pub fn trigger(&mut self) {
        self.state = EnvelopeState::Attack;
        self.time = 0.0;
    }

    pub fn release(&mut self) {
        if !matches!(self.state, EnvelopeState::Idle) {
            self.state = EnvelopeState::Release;
            self.time = 0.0;
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.time += 1.0 / SAMPLE_RATE as f32;
        match self.state {
            EnvelopeState::Attack => {
                self.level = (self.time / self.attack).min(1.0);
                if self.level >= 1.0 {
                    self.state = EnvelopeState::Decay;
                    self.time = 0.0;
                }
            }
            EnvelopeState::Decay => {
                self.level = 1.0 - (1.0 - self.sustain) * (self.time / self.decay).min(1.0);
                if self.level <= self.sustain {
                    self.state = EnvelopeState::Sustain;
                }
            }
            EnvelopeState::Sustain => {
                self.level = self.sustain;
            }
            EnvelopeState::Release => {
                self.level = self.sustain * (1.0 - self.time / self.release).max(0.0);
                if self.level <= 0.0 {
                    self.state = EnvelopeState::Idle;
                }
            }
            EnvelopeState::Idle => {
                self.level = 0.0;
            }
        }
        self.level
    }
}
