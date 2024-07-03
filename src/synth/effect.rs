use num_traits::Pow;

#[derive(Clone, Copy, Debug)]
pub enum Effect {
    Gain(f32),
    HardClip(f32),
    SoftCubic(f32),
    SoftExponential(f32),
    InfiniteClip(f32),
    BitCrusher(u32),
    Fangs(f32),
}

impl Effect {
    pub fn apply(self, sample: f32) -> f32 {
        match self {
            Self::Gain(level) => gain(sample, level),
            Self::HardClip(threshold) => hard_clip(sample, threshold),
            Self::SoftCubic(threshold) => soft_cubic(sample, threshold),
            Self::SoftExponential(threshold) => soft_exponential(sample, threshold),
            Self::InfiniteClip(amplitude) => infinite_clip(sample, amplitude),
            Self::BitCrusher(bits) => bit_crusher(sample, bits),
            Self::Fangs(threshold) => fangs(sample, threshold),
        }
    }
}

fn gain(input: f32, gain: f32) -> f32 {
    input * gain
}

fn hard_clip(input: f32, threshold: f32) -> f32 {
    if input > threshold {
        threshold
    } else if input < -threshold {
        -threshold
    } else {
        input
    }
}

fn soft_cubic(input: f32, threshold: f32) -> f32 {
    let a = threshold;
    input - a * (1./3.) * input.powf(3.)
}

fn soft_exponential(input: f32, threshold: f32) -> f32 {
    if input > 0. {
        threshold - (-input).exp()
    }
    else {
        -threshold + input.exp()
    }
}

fn infinite_clip(sample: f32, amplitude: f32) -> f32 {
    if sample >= 0. {amplitude} else {-amplitude}
}

fn bit_crusher(sample: f32, bits: u32) -> f32 {
    let amp_values = (2.pow(bits - 1)) as f32;
    (amp_values * sample).round() * (1. / amp_values)
}

fn fangs(input: f32, threshold: f32) -> f32 {
    if input.abs() < threshold {
        input
    } else {
        threshold * (3.0 - (input.abs() * 2.0 / threshold)) / 3.0 * input.signum()
    }
}