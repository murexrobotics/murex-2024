use super::Action;
use std::time::Duration;

pub struct Throttle {
    throttle: f32,
    interval: Duration
}

pub struct Stabilize;

impl Action for Throttle {
    async fn exec(self) {
        todo!()
    }
}

impl Action for Stabilize {
    async fn exec(self) {
        todo!()
    }
}

