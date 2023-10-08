#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_labels)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod input;
mod actions;
mod rov;

use log::{trace, info, warn, error, debug};

fn main() {
    env_logger::init();

    let rov = rov::Rov::new();
    let socket = 
        input::initialize_connection().unwrap();
    
    
    info!("Starting main loop");
    'running: loop {
        input::receive(&socket);
    }
}