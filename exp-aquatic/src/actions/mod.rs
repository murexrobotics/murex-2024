use std::time::Duration;

pub enum Actions {
    // positive throttle is forward, negative throttle is reverse
    // throttle must be between -1.0 and 1.0
    // out of bounds will be clamped
    Throttle(usize, f32), // (motor, throttle)
    // Throttle the motors over specified duration
    ThrottleOverTime(usize, f32, Duration), // (motor, throttle, duration)
    // Stabilize the sub using sensor feedback
    Stabilize,
    // Bring sub back to surface using sensor feedback
    ReSurface,
    // Move sub to specified depth using sensor feedback (meters)
    MoveToDepth(f32), // (depth)
    // Move sub to specified heading using sensor feedback (degrees)
    MoveToHeading(f32), // (heading)
    // TODO: Lock move to position behind feature flag
    // Move sub to specified position using sensor feedback (meters, meters, meters)
    MoveToPosition(f32, f32, f32), // (x, y, z) 
    // Re callibratre all sensors based on current position
    CallibrateSensors,
    // System diagnostics
    Report,
    // System shutdown
    Shutdown,
    // System reboot
    Reboot,
}