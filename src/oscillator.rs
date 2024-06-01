use std::f32::consts::PI;

use num_traits::Pow;

use crate::SAMPLE_RATE;

#[derive(Clone, Copy, Debug)]
pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    AnalogSawtooth,
    Exp,
}

#[derive(Clone, Copy, Debug)]
pub struct Oscillator {
    waveform: Waveform,
    frequency: f32,
    phase: f32,
    duty: f32,
    harmonics: u32,
}

impl Oscillator {
    pub fn new(waveform: Waveform, frequency: f32) -> Self {
        Self {
            waveform,
            frequency,
            phase: 0.0,
            duty: 0.5,              // duty cycle; only used for square waves
            harmonics: 50,          // number of harmonics summed; only used for sawtooth waves
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn frequency(&self) -> f32 {
        self.frequency
    }

    pub fn set_duty(&mut self, duty: f32) {
        self.duty = duty;
    }

    pub fn duty(&self) -> f32 {
        self.duty
    }

    pub fn set_harmonics(&mut self, harmonics: u32) {
        self.harmonics = harmonics;
    }

    pub fn harmonics(&self) -> u32 {
        self.harmonics
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = match self.waveform {
            Waveform::Sine => (2.0 * PI * self.phase).sin(),
            Waveform::Square => if self.phase < self.duty { 1.0 } else { -1.0 },
            Waveform::Triangle => if self.phase < 0.5 { 4.0 * self.phase - 1.0 } else { 3.0 - 4.0 * self.phase },
            Waveform::Sawtooth => 2.0 * self.phase - 1.0,
            Waveform::AnalogSawtooth => {
                let mut sample = 0.0;
                for k in 1..self.harmonics {
                    sample += (2.0 * PI * k as f32 * self.phase).sin() / k as f32;
                }
                -2.0/PI * sample
            }
            Waveform::Exp => (2. * self.phase - 1.).powf(3.) + 0.5
        };
        self.phase = (self.phase + self.frequency / SAMPLE_RATE as f32) % 1.0;
        sample
    }
}