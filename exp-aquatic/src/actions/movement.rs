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
        let pressure = rov.poll_ms5837();
        let deadzone = 0.1; //to be tuned with testing
        for i in 0..4 {
            rov.set_thruster(i, 0);
        }
        rov.set_thruster(4, 1);
        rov.set_thruster(5, 0);
        while pressure > deadzone {
            pressure = rov.poll_ms5837();
        }
        for i in 0..6 {
            rov.set_thruster(i, 0);
        }
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
