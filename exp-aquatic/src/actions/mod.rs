mod movement;
mod system;

use movement::*;
use enum_dispatch::enum_dispatch;


#[enum_dispatch(Actions)]
pub(crate) trait Action {
    async fn exec(self);
}

#[enum_dispatch]
pub enum Actions {
    Throttle,
    Stabilize,
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