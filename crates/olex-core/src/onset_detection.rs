pub struct OnsetDetector {
    previous: f32,
    cooldown: usize,
}

impl OnsetDetector {
    pub fn new() -> Self {
        Self {
            previous: 0.0,
            cooldown: 0,
        }
    }

    pub fn process(&mut self, env: f32) -> bool {
        if self.cooldown > 0 {
            self.cooldown -= 1;
            self.previous = env;
            return false;
        }

        let rise = env - self.previous;
        self.previous = env;

        if env > 0.03 && rise > 0.01 {
            self.cooldown = 2_000; // rough cooldown at 48k ≈ 41ms
            true
        } else {
            false
        }
    }
}
