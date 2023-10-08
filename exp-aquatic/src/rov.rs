use linux_embedded_hal::{I2cdev, Spidev};
use rppal::uart::Uart;

pub struct Rov {
    i2c: I2cdev,
    spi: Spidev,
    uart: Uart,

    pub thrusts: (f32, f32, f32, f32, f32, f32), // (front, back, left, right, top, bottom)
    
    pub acceleration: (f32, f32, f32), // (x, y, z)
    pub velocity: (f32, f32, f32), // (x, y, z)
    pub position: (f32, f32, f32), // (x, y, z)

    pub angular_velocity: (f32, f32, f32), // (x, y, z)
    pub orientation: (f32, f32, f32), // (x, y, z)

    pub magnetic_field: (f32, f32, f32), // (x, y, z)

    pub temperature: f32, // Celsius
    pub pressure: f32, // Pascals
    pub humidity: f32, // Percent
    pub gas_resistance: f32, // Ohms

    pub leak_matrix: (bool, bool, bool, bool), // (front, back, left, right)

    pub current: f32, // Amps
}

impl Rov {
    pub fn new() -> Self {
        todo!()
    }

    pub(crate) fn refresh(&mut self) {
        todo!()
    }

    pub(crate) fn poll_bme680(&self) -> (f32, f32, f32, f32) {
        todo!()
    }

    pub(crate) fn poll_bmi088(&self) -> ((f32, f32, f32), (f32, f32, f32)) {
        todo!()
    }

    pub(crate) fn poll_mmc5603(&self) -> (f32, f32, f32) {
        todo!()
    }

    pub(crate) fn poll_ms5387(&self) -> f32 {
        todo!()
    }

    pub(crate) fn poll_leak_matrix(&self) -> (bool, bool, bool, bool) {
        todo!()
    }

    pub(crate) fn poll_ina226(&self) -> f32 {
        todo!()
    }

    pub(crate) fn write_neopixel(&self, pixel: usize, color: (u8, u8, u8)) {
        todo!()
    }

    pub(crate) fn write_display(&self, text: &str) {
        todo!()
    }

    pub(crate) fn pwm_duty_cycle(&self, channel: usize, duty_cycle: f32) {
        todo!()
    }

    pub(crate) fn set_thruster(&self, thruster: usize, thrust: f32) {
        todo!()
    }
}