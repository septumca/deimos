pub struct Cooldown {
    remaining_time: f32,
    total_time: f32
}

impl Cooldown {
    pub fn new(time: f32) -> Cooldown {
        Cooldown {
            total_time: time,
            remaining_time: 0.0
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.remaining_time > 0.0 {
            self.remaining_time = self.remaining_time - delta_time;
        }
    }

    pub fn has_elapsed(&self) -> bool {
        self.remaining_time <= 0.0
    }

    pub fn reset(&mut self) {
        self.remaining_time = self.total_time;
    }
}