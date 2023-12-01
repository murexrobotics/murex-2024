// TODO: Adjust i2c adresses
// TODO: Adjust i2c bus
// TODO: Adjust sensor settings/ configs

use std::sync::mpsc::{Sender, Receiver};
use std::thread::JoinHandle;
use std::net::UdpSocket;
use std::time::Duration;

use bme680::{SettingsBuilder, Bme680, OversamplingSetting, IIRFilterSize, PowerMode, I2CAddress};
use embedded_hal::blocking::{i2c, delay::DelayMs};
use linux_embedded_hal::{Delay, I2cdev};
use lis3mdl_driver::{Lis3mdl, Address};
use shared_bus::BusManagerSimple;
// use ms5837;
// use rppal::gpio::Gpio;

use serde::Serialize;
use serde_json;

#[derive(Debug)]
pub enum TelemetryCommand {
    Stop,
    ReCalibrate,
}

#[derive(Serialize)]
pub struct TelemetryPacket {
    // BME680
    pub humidity: f32,
    pub temperature: f32,
    pub gas_resistance: u32,
    pub internal_pressure: f32,
    // MS5837
    pub external_pressure: f32,
    // BMI088
    pub acceleration: (f32, f32, f32),
    pub angular_velocity: (f32, f32, f32),
    // LIS3MDL
    pub magnetic_field: (i32, i32, i32),
}

pub struct Telemetry {
    handle: Option<JoinHandle<()>>,
    main_sender: Sender<TelemetryCommand>
}

impl Telemetry{
    pub fn start() -> (Receiver<TelemetryPacket>, Telemetry) {
        // TODO: Initialize MPSC channel
        // TODO: Spawn telemetry thread
        // TODO: Init BME680, BMI088, MS5837, LIS3MDL, PMW3901
        // TODO: Polling sensors in event loop
        todo!()
    }
}

impl Drop for Telemetry {
    fn drop(&mut self) {
        self.main_sender.send(TelemetryCommand::Stop).expect("Failed to send stop message");
        if let Some(handle) = self.handle.take() {
            handle.join().expect("Failed to join Telemetry thread");
        }
    }
}

impl Telemetry {
    pub fn calibrate(&mut self) {
        self.main_sender.send(TelemetryCommand::ReCalibrate).expect("Failed to send recalibrate message");
    }

    pub fn stop(&mut self) {
        self.main_sender.send(TelemetryCommand::Stop).expect("Failed to send stop message");
    }
}