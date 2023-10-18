use super::Action;
use crate::rov::Rov;

struct Reboot();
impl Action for Reboot {
    fn exec(self, rov: &mut Rov) {
        todo!()
    }
}

struct Shutdown();
impl Action for Shutdown {
    fn exec(self, rov: &mut Rov) {
        todo!()
    }
}

struct SystemReport();
impl Action for SystemReport {
    fn exec(self, rov: &mut Rov) {
    }
}

struct LeakReport();
impl Action for LeakReport {
    fn exec(self, rov: &mut Rov) {
        todo!()
    }
}

struct CallibrateSensors();
impl Action for CallibrateSensors {
    fn exec(self, rov: &mut Rov) {
        todo!()
    }
}