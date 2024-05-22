use std::f64::consts::PI;


struct Note {
    id: u32,
    on_time: f64,
    off_time: f64,
    active: bool,
    channel: u32,
}

impl Note {
    pub fn new() -> Self {
        Self {
            id: 0,
            on_time: 0.,
            off_time: 0.,
            active: false,
            channel: 0,
        }
    }
}

enum OscKind {
    Sine,
    Square,
    Triangle,
    AnalogSaw(u32),         
    DigitalSaw,
    Noise,
}

fn osc(freq: f64, time: f64, kind: OscKind) -> f64 {
    match kind {
        OscKind::Sine => (w(freq) * time).sin(),
        OscKind::Square => if (w(freq) * time).sin() > 0. {1.0} else {-1.0},
        OscKind::Triangle => ((w(freq) * time).sin()).asin() * (2.0 / PI),
        OscKind::AnalogSaw(precision) => {
            let mut output = 0.;
    
            for n in 1..precision {
                let n = n as f64;
                output += ((n * w(freq) * time).sin()) / n;
            }
            
            output * (2.0 / PI)
        }
        OscKind::DigitalSaw => (2.0 / PI) * (freq * PI * (time % (1.0 / freq)) - (PI / 2.0)),
        OscKind::Noise => 2.0 * rand::random::<f64>() - 1.0,
    }
}

trait Envelope {
    fn get_amplitude(&mut self, time: f64, time_on: f64, time_off: f64);
    fn get_sample(&mut self, time: f64, time_on: f64, time_off: f64);
}

pub struct EnvelopeADSR {
    attack_time: f64,
	decay_time: f64,
	sustain_amplitude: f64,
	release_time: f64,
	start_amplitude: f64,
}

impl EnvelopeADSR {
    pub fn new() -> Self {
        Self {
            attack_time: 0.30,
            decay_time: 0.01,
            start_amplitude: 1.0,
            sustain_amplitude: 0.8,
            release_time: 0.20,
        }
    }

    pub fn get_amplitude(&mut self, time: f64, time_on: f64, time_off: f64) -> f64 {
        let mut amplitude = 0.;
        let mut release_amplitude = 0.;
        
        if time_on > time_off {         // note is on
            let lifetime = time - time_on;

            if lifetime <= self.attack_time {
                // attack phase
                amplitude = (lifetime / self.attack_time) * self.start_amplitude;
            }

            if lifetime > self.attack_time && lifetime <= (self.attack_time + self.decay_time) {
                // decay phase
                amplitude = ((lifetime - self.attack_time) / self.decay_time) * (self.sustain_amplitude - self.start_amplitude) + self.start_amplitude;
            }

            if lifetime > (self.attack_time + self.decay_time) {
                // sustain phase
                amplitude = self.sustain_amplitude;
            }
        }

        else {              // note is off
            let lifetime = time_off - time_on;

            if lifetime <= self.attack_time {
                release_amplitude = (lifetime / self.attack_time) * self.start_amplitude;
            }

            if lifetime > self.attack_time && lifetime <= (self.attack_time + self.decay_time) {
                release_amplitude = ((lifetime - self.attack_time) / self.decay_time) * (self.sustain_amplitude - self.start_amplitude) + self.start_amplitude;
            }

            if lifetime > (self.attack_time + self.decay_time) {
                release_amplitude = self.sustain_amplitude;
            }

            amplitude = ((time - time_off) / self.release_time) * (0.0 - release_amplitude) + release_amplitude;
        }

        if amplitude <= 0.000 {
            amplitude = 0.;
        }

        amplitude
    }

    pub fn get_amplitude_u8(&mut self, time: f64, time_on: f64, time_off: f64) -> u8 {
        let amplitude = self.get_amplitude(time, time_on, time_off);
        let amplitude_shift = ((amplitude + 1.) / 2.) * 127.;
        amplitude_shift.round() as u8
    }

    pub fn get_sample(&mut self, time: f64, time_on: f64, time_off: f64) -> u8 {
        let amplitude = self.get_amplitude(time, time_on, time_off) * 1.0 * osc(440., time, OscKind::Sine);
        let sample = ((amplitude + 1.) / 2.) * 127.;
        sample.round() as u8
    }

    pub fn get_sample1(&mut self, time: f64, time_on: f64, time_off: f64) -> u8 {
        let amplitude = self.get_amplitude(time, time_on, time_off) * 1.0 * osc(440., time, OscKind::Sine);
        if amplitude > 0. {50} else {0}
    }
}

impl Envelope for EnvelopeADSR {
    
}


// ------------------- utilities  ------------------- 

fn w(freq: f64) -> f64 {
	freq * 2.0 * PI
}