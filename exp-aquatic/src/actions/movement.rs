use super::Action;
use std::time::Duration;

pub struct Throttle {
    throttle: f32,
    interval: Duration
}

impl Action for Throttle {
    async fn exec(self) {
        todo!()
    }
}

pub struct Stabilize;

impl Action for Stabilize {
    async fn exec(self) {
        todo!()
    }
}

