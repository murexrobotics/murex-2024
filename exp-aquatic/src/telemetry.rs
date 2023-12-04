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
use ms5837;
use pmw3901::Pmw3901;

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
        let (main_sender, main_receiver) = std::sync::mpsc::channel();
        let (sys_sender, sys_receiver) = std::sync::mpsc::channel();
        
        let handle = std::thread::spawn(move || {
            // ---------- Initialize I2C bus ----------
            let i2c = BusManagerSimple::new(I2cdev::new("/dev/i2c-1").expect("Failed to initialize I2C bus"));
            let mut delay = Delay {};

            // ---------- Initialize BME680 ----------
            // TODO: Adjust settings, play with oversampling and filter size
            let mut bme = Bme680::init(i2c.acquire_i2c(), &mut delay, I2CAddress::Primary).expect("Failed to initialize BME680");
            let settings = SettingsBuilder::new()
                .with_gas_measurement(Duration::from_millis(1500), 320, 25)
                .with_temperature_offset(-5.0)
                .with_run_gas(true)
                .build();
            bme.set_sensor_settings(&mut delay, settings).expect("Failed to set BME680 settings");

            // ---------- Initialize MS5837 ----------
            let mut ms = ms5837::new(i2c.acquire_i2c()).init().expect("Failed to initialize MS5837");

            // ---------- Initialize BMI088 ----------
            // TODO: Pending completion of driver *cough* *cough*

            // ---------- Initialize LIS3MDL ----------
            let mut lis = Lis3mdl::new(i2c.acquire_i2c(), Address::Addr1C).expect("Failed to initialize LIS3MDL");

            // ---------- Initialize PMW3901 ----------
            // TODO: Adjust bus and pin numbers
            let mut pmw = Pmw3901::new(0, 0).expect("Invalid SPI bus or pin numbers").init().expect("Failed to initialize PMW3901");

            // TODO: Implement event loop
            'telemetry: loop {
                if let Ok(cmd) = main_receiver.try_recv() {
                    match cmd {
                        TelemetryCommand::Stop => break 'telemetry,
                        _ => todo!("Implement TelemetryCommands"),
                    }
                }

                // TODO: Read data from sensors
                let telemetry = TelemetryPacket {
                    humidity: 0.0,
                    temperature: 0.0,
                    gas_resistance: 0,
                    internal_pressure: 0.0,
                    external_pressure: 0.0,
                    acceleration: (0.0, 0.0, 0.0),
                    angular_velocity: (0.0, 0.0, 0.0),
                    magnetic_field: (0, 0, 0),
                };

                sys_sender.send(telemetry).expect("Failed to send telemetry packet");
            }
        });

        (
            sys_receiver, 
            Telemetry {
                handle: Some(handle),
                main_sender
            }
        )
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