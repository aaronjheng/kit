use std::time;

pub struct ResourceUse {
    start: time::Instant,
    end: time::Instant,
}

impl ResourceUse {
    pub fn new() -> Self {
        let now = time::Instant::now();

        Self {
            start: now,
            end: now,
        }
    }

    pub fn begin(&mut self) {
        self.start = time::Instant::now();
    }

    pub fn finish(&mut self) {
        self.end = time::Instant::now();
    }

    pub fn elapsed(&self) -> time::Duration {
        self.end - self.start
    }
}

mod resource;
