pub struct EnvelopeFollower {
    attack_coeff: f32,
    release_coeff: f32,
    value: f32,
}

impl EnvelopeFollower {
    pub fn new(sample_rate: f32, attack_ms: f32, release_ms: f32) -> Self {
        fn coeff(sample_rate: f32, ms: f32) -> f32 {
            (-1.0 / (0.001 * ms * sample_rate)).exp()
        }

        Self {
            attack_coeff: coeff(sample_rate, attack_ms),
            release_coeff: coeff(sample_rate, release_ms),
            value: 0.0,
        }
    }

    pub fn process(&mut self, input_abs: f32) -> f32 {
        let coeff = if input_abs > self.value {
            self.attack_coeff
        } else {
            self.release_coeff
        };

        self.value = input_abs + coeff * (self.value - input_abs);
        self.value
    }
}
