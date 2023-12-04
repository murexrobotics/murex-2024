#![allow(unused_must_use, dead_code, unused_imports)]
// #![no_std]
#![forbid(unsafe_code)]

mod consts;

use std::fmt::{write, Display};
use std::thread::sleep;
use std::time::Duration;

use consts::*;
use embedded_hal::blocking::{
    delay::DelayUs,
    i2c::{Read, Write, WriteRead},
};

#[derive(Debug)]
pub enum Error {
    Temp,
    WriteError,
    ReadError,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Temp => write!(f, "Placeholder Error"),
            Error::ReadError => write!(f, "Read Error"),
            Error::WriteError => write!(f, "Write Error"),
        }
    }
}
impl std::error::Error for Error {}

// TODO: Look at MODE2 register for init
pub struct PCA9685<I2C> {
    i2c_handle: I2C,
    address: u8,
}

impl<I2C> PCA9685<I2C>
where
    I2C: Read + Write + WriteRead,
{
    pub fn new(i2c_handle: I2C) -> Self {
        Self {
            i2c_handle,
            address: PCA9685_I2C_ADDR,
        }
    }
    pub fn restart(&mut self) -> Result<(), Error> {
        // TODO: Check to see if this is necessary and more so it's purpose
        // Datasheet p.15
        let mut buffer = [0; 1];
        self.i2c_handle
            .write_read(self.address, &[MODE1], &mut buffer)
            .map_err(|_| Error::ReadError)?;

        // Verify that RESTART bit is set
        if buffer[0] & 0x10 == 0x10 {
            // Bit Mask to clear bit 4, SLEEP bit
            self.i2c_handle
                .write(self.address, &[MODE1, buffer[0] & 0xEF])
                .map_err(|_| Error::WriteError)?;
            // Wait 500us for oscillator to stabilize
            sleep(Duration::from_micros(500));
            // Write logic 1 to bit 7, RESTART bit
            self.i2c_handle
                .write(self.address, &[MODE1, buffer[0] | 0x10])
                .map_err(|_| Error::WriteError)?;
        }
        Ok(())
    }

    pub fn set_pwm_freq(&mut self, frequency: u32) -> Result<(), Error> {
        // Datasheet p.25
        let mut buffer: [u8; 1] = [0; 1];
        self.i2c_handle
            .write_read(self.address, &[MODE1], &mut buffer)
            .map_err(|_| Error::ReadError)?;

        let mode = buffer[0];
        println!("{mode:b}");

        self.i2c_handle
            .write(self.address, &[MODE1, mode & 0x10])
            .map_err(|_| Error::WriteError)?;
        println!("{:b}", mode | 0x10);

        let prescale_value = (25_000_000 / (4096 * frequency)) as u8; // TODO: Add -1 back
        self.i2c_handle
            .write(self.address, &[PRE_SCALE, prescale_value])
            .map_err(|_| Error::WriteError)?; // TODO: Handle result

        let mut buffer2: [u8; 1] = [0; 1];
        self.i2c_handle
            .write_read(self.address, &[MODE1], &mut buffer2)
            .map_err(|_| Error::ReadError)?;

        let mode2 = buffer2[0];
        println!("{mode2:b}");

        self.i2c_handle
            .write(self.address, &[MODE1, mode])
            .map_err(|_| Error::WriteError)?;
        Ok(())
    }

    pub fn set_duty_cycle(&mut self, channel: u8, duty_cycle: u16) -> Result<(), Error> {
        if channel > 15 {
            return Err(Error::Temp); // TODO: Come up with proper error type
        } else if duty_cycle > 4095 {
            return Err(Error::Temp); // TODO: Come up with proper error type
        }

        // Datasheet p.16
        self.i2c_handle
            .write(
                self.address,
                &[
                    LED14_ON_L,              //LED0_ON_L + (channel * 4), // Channel register
                    0x00,                    // Start PWM at 0, start of every cycle
                    0x00,                    // Start PWM at 0, start of every cycle
                    duty_cycle as u8,        // LSB for stop bits
                    (duty_cycle >> 8) as u8, // MSB for stop bits
                ],
            )
            .map_err(|_| Error::WriteError) // Handle Result
    }
}
