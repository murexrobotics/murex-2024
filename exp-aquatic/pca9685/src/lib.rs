#![allow(unused_must_use, dead_code, unused_imports)]
// #![no_std]
#![forbid(unsafe_code)]

mod consts;

use std::fmt::Display;
use embedded_hal::blocking::{
    i2c::{
        Write, 
        Read,
        WriteRead
    },
    delay::DelayUs
};
use consts::*;

#[derive(Debug)]
pub enum Error {
    Temp
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Temp => write!(f, "Placeholder Error")
        }
    }
}
impl std::error::Error for Error {}

// TODO: Look at MODE2 register for init
pub struct PCA9685<I2C, T> {
    i2c_handle: I2C,
    time: T,
    address: u8,
}

impl<I2C, T> PCA9685<I2C, T> 
where 
    I2C: Read + Write + WriteRead,
    T: DelayUs<u16>
{
    pub fn new(i2c_handle: I2C, time: T) -> Self {
        Self {
            i2c_handle,
            address: PCA9685_I2C_ADDR, 
            time
        }
    }
    pub fn restart(&mut self) -> Result<(), Error> {
        // TODO: Check to see if this is necessary and more so it's purpose
        // Datasheet p.15
        let mut buffer = [0; 1];
        self.i2c_handle.write_read(self.address, &[MODE1], &mut buffer);

        // Verify that RESTART bit is set
        if buffer[0] & 0x10 == 0x10 {
            // Bit Mask to clear bit 4, SLEEP bit
            self.i2c_handle.write(self.address, &[MODE1, buffer[0] & 0xEF]); 
            // Wait 500us for oscillator to stabilize
            self.time.delay_us(500);
            // Write logic 1 to bit 7, RESTART bit
            self.i2c_handle.write(self.address, &[MODE1, buffer[0] | 0x10]);
        }
        Ok(())
    }

    pub fn set_pwm_freq(&mut self, frequency: u32) -> Result<(), Error> {
        // Datasheet p.25
        let prescale_value = (25_000_000 / (4096 * frequency) - 1) as u8;
        self.i2c_handle.write(self.address, &[PRE_SCALE, prescale_value]); // TODO: Handle result
        Ok(())
    }

    pub fn set_duty_cycle(&mut self, channel: u8, duty_cycle: u16) -> Result<(), Error> {
        if channel > 15 {
            return Err(Error::Temp); // TODO: Come up with proper error type
        }

        else if duty_cycle > 4095 {
            return Err(Error::Temp); // TODO: Come up with proper error type
        }

        // Datasheet p.16
        self.i2c_handle.write(
            self.address, 
            &[
                LED0_ON_L + (channel * 4), // Channel register
                0x00, // Start PWM at 0, start of every cycle
                0x00, // Start PWM at 0, start of every cycle
                (duty_cycle & 0xFF) as u8, // LSB for stop bits
                (duty_cycle >> 8) as u8 // MSB for stop bits
            ]
        ); // Handle Result
        Ok(())
    }

}