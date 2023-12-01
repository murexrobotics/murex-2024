use std::sync::mpsc::{Sender, Receiver};
use std::thread::JoinHandle;
use std::net::UdpSocket;

use ahrs::{Ahrs, Madgwick};
use nalgebra::{Vector3, Quaternion};

use crate::telemetry::TelemetryPacket;


#[derive(Debug)]
pub enum SystemCommand {
    MoveTo(f32, f32, f32),
    TurnTo(f32),
    MoveBy(f32, f32, f32),
    TurnBy(f32),
    Stop,
    Surface,
    Stability
}

pub struct System {
    handle: Option<JoinHandle<()>>,
    main_sender: Sender<SystemCommand>,
}

impl System {
    pub fn start(tel_receiver: Receiver<TelemetryPacket>) -> System {
        // TODO: Init MPSC channels
        // TODO: calculate AHRS + Position when telemetry packet is received
        // TODO: Task execution event loop
        todo!()
    }
}

impl Drop for System {
    fn drop(&mut self) {
        self.main_sender.send(SystemCommand::Stop).expect("Failed to send stop message");
        if let Some(handle) = self.handle.take() {
            handle.join().expect("Failed to join Telemetry thread");
        }
    }
}