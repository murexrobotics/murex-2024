use super::Action;
use crate::rov::Rov;

struct MoveToHeading(f32);
impl Action for MoveToHeading {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}

struct MoveToDepth(f32);
impl Action for MoveToDepth {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}

struct MoveToPosition(f32, f32, f32);
impl Action for MoveToPosition {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}

struct MoveToRelativeHeading(f32);
impl Action for MoveToRelativeHeading {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}

struct MoveToRelativeDepth(f32);
impl Action for MoveToRelativeDepth {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}

struct MoveToRelativePosition(f32, f32, f32);
impl Action for MoveToRelativePosition {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}

struct ThrusterControl(usize, f32);
impl Action for ThrusterControl {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}

struct Resurface();
impl Action for Resurface {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}

struct EmergencySurface();
impl Action for EmergencySurface {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}

struct EmergencyStop();
impl Action for EmergencyStop {
    fn exec(&self, rov: Rov) {
        for i in 0..6 {
            rov.set_thruster(i, 0);
        }
    }
}

struct ToggleStabilityControl(bool);
impl Action for ToggleStabilityControl {
    fn exec(&self, rov: Rov) {
        todo!()
    }
}
