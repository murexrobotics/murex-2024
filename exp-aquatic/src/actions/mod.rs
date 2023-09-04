

mod movement;
mod system;

use movement::*;
use system::*;

use enum_dispatch::enum_dispatch;
use async_trait::async_trait;



#[enum_dispatch(Actions)]
pub(crate) trait Action {
    async fn exec(self);
}

#[enum_dispatch]
pub enum Actions {
    Throttle,
    // Stabilize,
    // Surface,
    // MoveToDepth,
    // TurnToAngle, 
    // TurnByAngle,
    // MoveToPosition,
    // CallibrateSensors,
    // Report,
    // Shutdown,
    // Reboot,
}