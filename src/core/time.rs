use std::time::{Duration, Instant};
use bevy_ecs::prelude::*;

pub struct Time {
    start_time: Instant,
    last_update_time: Option<Instant>,
    pub delta_seconds: f32
}

impl Default for Time {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            last_update_time: None,
            delta_seconds: Duration::from_secs(0).as_secs_f32()
        }
    }
}

impl Time {
    pub fn update(&mut self) {
        let now = Instant::now();
        if let Some(last_update_instant) = self.last_update_time {
            self.delta_seconds = (now - last_update_instant).as_secs_f32();
        } else {
            self.delta_seconds = (now - self.start_time).as_secs_f32();
        }
        self.last_update_time = Some(now);
    }
}

pub fn time_system(mut time: ResMut<Time>) {
    time.update()
}