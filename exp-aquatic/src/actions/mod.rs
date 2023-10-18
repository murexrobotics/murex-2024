mod movement;
mod system;

use crate::rov::Rov;

pub trait Action {
    fn exec(&self, rov: &mut Rov);
}