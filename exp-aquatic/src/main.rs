#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod socket;
mod system;
mod telemetry;
mod fusion;

use system::System;
use telemetry::Telemetry;

fn main() {
    let socket = socket::initialize_connection().unwrap();

    let (tel_sys_channel, tel) = Telemetry::start();

    loop {}
}
