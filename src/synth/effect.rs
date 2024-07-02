#[derive(Clone, Copy, Debug)]
pub enum Effect {
    Gain(f32),
    HardClip(f32),
    SoftCubic(f32),
}

impl Effect {
    pub fn apply(self, sample: f32) -> f32 {
        match self {
            Self::Gain(level) => gain(sample, level),
            Self::HardClip(threshold) => hard_clip(sample, threshold),
            Self::SoftCubic(threshold) => soft_cubic(sample, threshold),
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
    if input.abs() < threshold {
        input
    } else {
        threshold * (3.0 - (input.abs() * 2.0 / threshold)) / 3.0 * input.signum()
    }
}