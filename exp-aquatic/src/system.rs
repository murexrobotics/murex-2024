use std::net::UdpSocket;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

use ahrs::{Ahrs, Madgwick};
use nalgebra::{Quaternion, Vector3};

use crate::telemetry::TelemetryPacket;
use crate::fusion;

const SAMPLE_RATE: f32 = 0.1;
const BETA: f32 = 0.1;

#[derive(Debug)]
pub enum SystemCommand {
    MoveTo(f32, f32, f32),
    TurnTo(f32),
    MoveBy(f32, f32, f32),
    TurnBy(f32),
    SetThrusters(f32, f32, f32, f32, f32, f32),
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
            let (mut ahrs, mut pos) = fusion::init_sensor_fusion();
            let (mut target_x, target_y, target_z) = (0, 0, 0);
            let (mut target_pitch, mut target_roll, mut target_yaw) = (0, 0, 0);
            
            let (mut fr, mut fl, mut br, mut bl, mut u1, mut u2) = (0, 0, 0, 0, 0, 0);

            'system: loop {
                // Block thread until next telemetry packet is received, 
                // sensor fusion fails if there is no data
                let tel = tel_receiver
                    .recv()
                    .expect("Failed to receive telemetry packet");

                let (roll, pitch, yaw) = ahrs(&tel);
                let (x, y, z) = pos(&tel);

                // Non-blocking check to see if there is a new command to be executed
                if let Ok(cmd) = main_receiver.try_recv() {
                    match cmd {
                        SystemCommand::Stop => break 'system,
                        _ => todo!("Implement SystemCommands"),
                    }
                }

                

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
