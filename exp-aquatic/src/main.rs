#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod system;
mod telemetry;
mod socket;

use telemetry::Telemetry;
use system::System;

fn main() {
    let socket = socket::initialize_connection().unwrap();

    let (tel_sys_channel, tel) = Telemetry::start();

    loop {

    }
}