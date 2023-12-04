use std::net::UdpSocket;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use ahrs::{Ahrs, Madgwick};
use nalgebra::{Quaternion, Vector3};

use crate::telemetry::TelemetryPacket;

const SAMPLE_RATE: f32 = 0.1;
const BETA: f32 = 0.1;

#[derive(Debug)]
pub enum SystemCommand {
    MoveTo(f32, f32, f32),
    TurnTo(f32),
    MoveBy(f32, f32, f32),
    TurnBy(f32),
    Stop,
    Surface,
    Stability,
}

pub struct System {
    handle: Option<JoinHandle<()>>,
    main_sender: Sender<SystemCommand>,
}

impl System {
    pub fn start(tel_receiver: Receiver<TelemetryPacket>) -> System {
        let (main_sender, main_receiver) = std::sync::mpsc::channel();
        let handle = std::thread::spawn(move || {
            let mut ahrs = Madgwick::new(SAMPLE_RATE, BETA);

            loop {
                let tel = tel_receiver
                    .recv()
                    .expect("Failed to receive telemetry packet");

                let accel =
                    Vector3::new(tel.acceleration.0, tel.acceleration.1, tel.acceleration.2);
                // Gyro data needs to be in radians, fix if necessarry
                let gyro = Vector3::new(
                    tel.angular_velocity.0,
                    tel.angular_velocity.1,
                    tel.angular_velocity.2,
                );
                // Magnetometer data is in gauss, TODO: convert to right units, unsure if it is Tesla's or not
                let mag = Vector3::new(
                    tel.magnetic_field.0 as f32,
                    tel.magnetic_field.1 as f32,
                    tel.magnetic_field.2 as f32,
                );

                let quat = ahrs
                    .update(&gyro, &accel, &mag)
                    .expect("Failed to update AHRS filter");

                let (roll, pitch, yaw) = quat.euler_angles();
            }
        });
        System {
            handle: Some(handle),
            main_sender,
        }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        self.main_sender
            .send(SystemCommand::Stop)
            .expect("Failed to send stop message");
        if let Some(handle) = self.handle.take() {
            handle.join().expect("Failed to join Telemetry thread");
        }
    }
}
