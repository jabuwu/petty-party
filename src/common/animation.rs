pub struct Animation {
    finished: bool,
    time: f32,
}

impl Animation {
    pub fn stub() -> Self {
        Self {
            time: 0.5,
            finished: false,
        }
    }
    pub fn time(time: f32) -> Self {
        Self {
            time,
            finished: false,
        }
    }
    pub fn update(&mut self, seconds: f32) {
        self.time -= seconds;
    }
    pub fn finished(&mut self) -> bool {
        if !self.finished && self.time <= 0. {
            self.finished = true;
            true
        } else {
            false
        }
    }
}
