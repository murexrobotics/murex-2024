use linux_embedded_hal::{I2cdev, Spidev};
use rppal::uart::{Uart, Parity};
use anyhow::{Result, bail};
use log::{trace, info, warn, error, debug};


use std::time::Duration;

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
        // TODO: Initialize I2C, SPI, and UART
        // TODO: Remove .unwrap() calls and do propper error handling
        let mut uart = Uart::new(
            115_200, 
            Parity::None, 
            8, 
            1)
            .unwrap();

        uart.set_read_mode(1, Duration::default()).unwrap();
        uart.set_write_mode(true).unwrap();
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

    pub(crate) fn poll_ms5837(&self) -> f32 {
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

    /// Sets the thrust of a given thruster using MASCP
    /// 
    /// # Arguments
    ///    * `thruster` - The thruster to set the thrust of
    ///    * `thrust` - The thrust to set the thruster to (-1.0 - 1.0)
    /// 
    /// # Returns
    ///   * `Result<()>` - Ok if the thruster was set successfully, otherwise an error
    pub(crate) fn set_thruster(&mut self, thruster: usize, thrust: f32) -> Result<()>  {
        // TODO: Clarify structure for M[3] and M[4]
        // MASCP Thruster Write
        // M[0] = MASCP_THRUSTER_WRITE_INSTRUCTION
        // M[1] = THRUSTER_SELECT = 0x00 - 0x05 (Thruster 0 - 5)
        // M[2] = POSITIVE_THRUST = 0x00 - 0xFF (Thrust 0 - 255)
        // M[3] = NEGATIVE_THRUST = 0x00 - 0xFF (Thrust 0 - 255)
        // TODO: M[3] = INTERPOLATION_TIME
        // TODO: M[4] = INTERPOLATION_FUNCTION_SELECT
        // M[5-8] = 0xFF (Padding, later possibly checksum)

        // Check if a valid thruster is being selected
        if thruster > 5 || thruster < 0 {
            warn!(target: "thruster_events", "Thruster `{}` is out of range", thruster);
            bail!("Invalid thruster selected");
        }
        
        // clamp thrust value between 0 and 1, then scale to 0 - 255
        let thrust = thrust.max(-1.0).min(1.0) * 255.0;
        trace!(target: "thruster_events", "Setting thruster {} to {}", thruster, thrust);
        
        // Send thruster command
        let msg = [
            0x00, 
            thruster as u8, 
            thrust as u8,           // Positive thrust
            (-thrust) as u8,        // Negative thrust
            0x00,                   // TBD interpolation time
            0x00,                   // TBD interpolation function
            0xFF,                   // Padding
            0xFF,                   // Padding
        ];
        self.uart.write(&msg);
        trace!(target: "thruster_events", "Sending MASCP thruster command payload");

        // Validate thruster
        let mut buffer = [0u8; 8];
        // buffer [0] should be MASCP Thruster Write Response
        // buffer [1-5] should be equivalent to the message
        // buffer [6-7] should be 0x00
        // TODO: Do additional checks as necessary, even these check is probably overkill
        self.uart.read(&mut buffer);
        if buffer[0] != 0x01 || // MASCP Thruster Write Response
            buffer[6] != 0x00 || // ESC Padding is 0x00
            buffer[7] != 0x00   // ESC Padding is 0x00
        {
            bail!("Thruster write failed")
        }
        Ok(())
    }
}