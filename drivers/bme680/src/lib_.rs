#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::{thread, sync::Arc};
use std::time::Duration;

use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use i2cdev::core::I2CDevice;

const BME680_I2C_ADDR: u16 = 0x77;
const BME680_ID: u8 = 0x61;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum Register {
    STATUS = 0x73,
    RESET = 0xE0,
    ID = 0xD0,
    CONFIG = 0x75,
    CTRL_MEAS = 0x74,
    CTRL_HUM = 0x72,
    CTRL_GAS_1 = 0x71,
    CTRL_GAS_0 = 0x70,
    GAS_WAIT_0 = 0x64,
    RES_HEAT_0 = 0x5A,
    IDAC_HEAT_0 = 0x50,
    GAS_R_LSB = 0x2B,
    GAS_R_MSB = 0x2A,
    HUM_LSB = 0x26,
    HUM_MSB = 0x25,
    TEMP_XLSB = 0x24,
    TEMP_LSB = 0x23,
    TEMP_MSB = 0x22,
    PRESS_XLSB = 0x21,
    PRESS_LSB = 0x20,
    PRESS_MSB = 0x1F,
    MEAS_STATUS_0 = 0x1D,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum HumidityCalibrationCoefficients {
    H12_LSB = 0xE2, // Datasheet p.20
    H1_MSB = 0xE3, // 0xE2<3:0> / 0xE3 (LSB/MSB)
    H2_MSB = 0xE1, // 0xE2<7:4> / 0xE1 (LSB/MSB)
    H3 = 0xE4,
    H4 = 0xE5,
    H5 = 0xE6,
    H6 = 0xE7,
    H7 = 0xE8,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum PressureCalibrationCoefficients {
    P1_LSB = 0x8E,
    P1_MSB = 0x8F,
    P2_LSB = 0x90,
    P2_MSB = 0x91,
    P3 = 0x92,
    P4_LSB = 0x94,
    P4_MSB = 0x95,
    P5_LSB = 0x96,
    P5_MSB = 0x97,
    P6 = 0x99,
    P7 = 0x98,
    P8_LSB = 0x9C,
    P8_MSB = 0x9D,
    P9_LSB = 0x9E,
    P9_MSB = 0x9F,
    P10 = 0xA0
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum TemperatureCalibrationCoefficients {
    T1_LSB = 0xE9,
    T1_MSB = 0xEA,
    T2_LSB = 0x8A,
    T2_MSB = 0x8B,
    T3 = 0x8C,
}

#[derive(Debug)]
enum BmeError<T: I2CDevice> {
    WrongId,
    Unreachable,
    MeasuredDataNotReady,
    I2CError(T::Error),
}

struct Bme680<T: I2CDevice + Sized> {
    // I2C handle to communicate with BME680
    i2c_handle: T,
    // 3 Bit value to determine the oversampling for pressure
    pressure_oversampling: u8,
    // 3 Bit value to determine the oversampling for temperature
    temperature_oversampling: u8,
    // 3 Bit value to determine the oversampling for humidity
    humidity_oversampling: u8,
    // 3 Bit value to determine the filter coefficient
    filter: u8,

    raw_temperature: u32,
    raw_pressure: u32,
    raw_humidity: u32,
    t_fine: i32,

    pub temperature: i32,
    pub pressure: i32,
    pub humidity: i32,
}

impl<T: I2CDevice + Sized> Bme680<T> {
    pub fn new(mut i2c_handle: T) -> Result<Self, BmeError<T>> {
        // Soft Reset, needed when powering on/ initializing the sensor. Datasheet p.53
        println!("Resetting sensor");
        if let Err(error) = i2c_handle.smbus_write_byte_data(Register::RESET as u8, 0xB6) {
            println!("{:?}", error);
            return Err(BmeError::I2CError(error));
        }
        thread::sleep(Duration::from_millis(5));

        // Verify that correct sensor is connected
        println!("Verifying sensor ID");
        match i2c_handle.smbus_read_byte_data(Register::ID as u8) {
            Ok(id) => if id != BME680_ID {
                println!("Wrong sensor ID: {}", id);
                return Err(BmeError::WrongId);
            },
            Err(error) => return Err(BmeError::I2CError(error)),
        }
        thread::sleep(Duration::from_millis(5));

        // Set up heater and gaw wait settings
        // // TODO: Experiment with this value
        // if let Err(error) = i2c_handle.smbus_write_byte_data(Register::GAS_WAIT_0 as u8, 0x65) {
        //     return Err(BmeError::I2CError(error));
        // } 
        // thread::sleep(Duration::from_millis(5));

        // // TODO: Experiment with this value
        // if let Err(error) = i2c_handle.smbus_write_byte_data(Register::RES_HEAT_0 as u8, 0x73) {
        //     return Err(BmeError::I2CError(error));
        // }

        Ok( Self {
            i2c_handle,
            // Default value for pressure oversampling is 0b101/ 16x
            pressure_oversampling: 0b101,
            // Default value for temperature oversampling is 0b010/ 2x
            temperature_oversampling: 0b010,
            // Default value for humidity oversampling is 0b001/ 1x
            humidity_oversampling: 0b001,
            // Default value for filter is 0b010/ 3
            filter: 0b010,

            raw_temperature: 0,
            raw_pressure: 0,
            raw_humidity: 0,
            t_fine: 0,
            temperature: 0,
            pressure: 0,
            humidity: 0,
        })
    }

    pub fn read_sensor_data(&mut self) -> Result<(), BmeError<T>> {
        // TODO: Add gas measurement protocols: p.16
        // Configure filter settings, datasheet p. 28
        self.write_byte(Register::CONFIG as u8, self.filter << 2)?;

        // Configure Humidity oversampling, datasheet p. 28
        self.write_byte(Register::CTRL_HUM as u8, self.humidity_oversampling)?;
        
        // Configure Temperature and Pressure oversampling and mode setting, datasheet p. 28
        let ctrl_meas: u8 = self.temperature_oversampling << 5 | self.pressure_oversampling << 2 | 0b1; 
        self.write_byte(Register::CTRL_MEAS as u8, ctrl_meas)?;

        // Read status register to check if data is ready, datasheet p. 36
        while let Ok(status) = self.read_byte(Register::MEAS_STATUS_0 as u8) {
            if status >> 7 == 0x1 { break }
            thread::sleep(Duration::from_millis(5));
        }

        // Read raw data from sensor
        let hum_lsb = self.read_byte(Register::HUM_LSB as u8)?;
        let hum_msb = self.read_byte(Register::HUM_MSB as u8)?;
        self.raw_humidity = (hum_msb as u32) << 8 | hum_lsb as u32; // Technically only needs 16 bits but this is for safety

        let temp_msb = self.read_byte(Register::TEMP_MSB as u8)?;
        let temp_lsb = self.read_byte(Register::TEMP_LSB as u8)?;
        let temp_xlsb = self.read_byte(Register::TEMP_XLSB as u8)?;
        self.raw_temperature = (temp_msb as u32) << 12 | (temp_lsb as u32) << 4 | (temp_xlsb as u32) >> 4; // Technically only needs 20 bits but 32 is smallest size that works

        let press_msb = self.read_byte(Register::PRESS_MSB as u8)?;
        let press_lsb = self.read_byte(Register::PRESS_LSB as u8)?;
        let press_xlsb = self.read_byte(Register::PRESS_XLSB as u8)?;
        self.raw_pressure = (press_msb as u32) << 12 | (press_lsb as u32) << 4 | (press_xlsb as u32) >> 4; // Technically only needs 20 bits but 32 is smallest size that works

        // Calibrate raw values
        // Order is important, temperature must be calibrated first
        let (t_fine, temp) = self.calculate_temperature()?;
        self.t_fine = t_fine;
        self.temperature = temp;

        // Pressure depends on t_fine
        let pressure = self.calculate_pressure()?;
        self.pressure = pressure;

        // Humidity depends on calibrated temperature
        let humidity = self.calculate_humidity()?;
        self.humidity = humidity;

        Ok(())
    }
   
    fn calculate_temperature(&mut self) -> Result<(i32, i32), BmeError<T>> {
        let t1 = (self.read_byte(TemperatureCalibrationCoefficients::T1_MSB as u8)? as u32) << 8 | self.read_byte(TemperatureCalibrationCoefficients::T1_LSB as u8)? as u32;
        let t2 = (self.read_byte(TemperatureCalibrationCoefficients::T2_MSB as u8)? as u32) << 8 | self.read_byte(TemperatureCalibrationCoefficients::T2_LSB as u8)? as u32;
        let t3 = self.read_byte(TemperatureCalibrationCoefficients::T3 as u8)? as u32;

        let var1 = (self.raw_temperature as i32 >> 3) - ((t1 as i32) << 1);
        let var2 = (var1 * t2 as i32) >> 11;
        let var3 = (((var1 >> 1) * (var1 >> 1)) >> 12) * ((t3 as i32) << 4 ) >> 14;
        let t_fine = var2 + var3;
        let temp_comp = (t_fine * 5 + 128) >> 8;

        Ok((t_fine, temp_comp))
    }

    fn calculate_pressure(&mut self) -> Result<i32, BmeError<T>> {
        let p1 = (self.read_byte(PressureCalibrationCoefficients::P1_MSB as u8)? as u32) << 8 | self.read_byte(PressureCalibrationCoefficients::P1_LSB as u8)? as u32;
        let p2 = (self.read_byte(PressureCalibrationCoefficients::P2_MSB as u8)? as u32) << 8 | self.read_byte(PressureCalibrationCoefficients::P2_LSB as u8)? as u32;
        let p3 = self.read_byte(PressureCalibrationCoefficients::P3 as u8)? as u32;
        let p4 = (self.read_byte(PressureCalibrationCoefficients::P4_MSB as u8)? as u32) << 8 | self.read_byte(PressureCalibrationCoefficients::P4_LSB as u8)? as u32;
        let p5 = (self.read_byte(PressureCalibrationCoefficients::P5_MSB as u8)? as u32) << 8 | self.read_byte(PressureCalibrationCoefficients::P5_LSB as u8)? as u32;
        let p6 = self.read_byte(PressureCalibrationCoefficients::P6 as u8)? as u32;
        let p7 = self.read_byte(PressureCalibrationCoefficients::P7 as u8)? as u32;
        let p8 = (self.read_byte(PressureCalibrationCoefficients::P8_MSB as u8)? as u32) << 8 | self.read_byte(PressureCalibrationCoefficients::P8_LSB as u8)? as u32;
        let p9 = (self.read_byte(PressureCalibrationCoefficients::P9_MSB as u8)? as u32) << 8 | self.read_byte(PressureCalibrationCoefficients::P9_LSB as u8)? as u32;
        let p10 = self.read_byte(PressureCalibrationCoefficients::P10 as u8)? as u32;

        // Look at datasheet p. 18-19 for explanation of the following calculations
        let mut var1 = (self.t_fine as i32 >> 1) - 64000;
        let mut var2 = (((var1 >> 2) * (var1 >> 2)) >> 11) * (p6 as i32);
        var2 = var2 + ((var1 * p5 as i32) << 1);
        var2 = (var2 >> 2) + (p4 as i32) << 16;
        var1 = (((((var1 >> 2) * (var1 >> 2)) >> 13) * ((p3 as i32) << 5)) >> 3) + (((p2 as i32) * var1) >> 1 );
        var1 = var1 >> 18;
        var1 = ((32768 + var1) * (p1 as i32)) >> 15;
        let mut press_comp =  1048576 - self.raw_pressure;
        press_comp = (press_comp - (var2 as u32 >> 12)) * 3125;
        if press_comp >= ( 1 <<30 ) {
            press_comp = (press_comp / var1 as u32) << 1;
        } else {
            press_comp = (press_comp << 1) / var1 as u32;
        }
        var1 = (p9 as i32) * (((press_comp as i32 >> 3) * (press_comp as i32 >> 3)) >> 13);
        var2 = ((press_comp as i32 >> 2) * (p8 as i32)) >> 13;
        let var3 = ((press_comp as i32 >> 8) * (press_comp as i32 >> 8) * (press_comp as i32 >> 8) * (p10 as i32)) >> 17;
        Ok(press_comp as i32 + ((var1 + var2 + var3 + ((p7 as i32) << 7)) >> 4))
    }

    fn calculate_humidity(&mut self) -> Result<i32, BmeError<T>> {
        let h12_lsb = self.read_byte(HumidityCalibrationCoefficients::H12_LSB as u8)? as u32;
        let mut h1 = (self.read_byte(HumidityCalibrationCoefficients::H1_MSB as u8)? << 4) as u32;
        let mut h2 = (self.read_byte(HumidityCalibrationCoefficients::H2_MSB as u8)? << 4) as u32;

        h1 |= h12_lsb & 0x0F; // Bit mask to extract only the first 4 bits
        h2 |= h12_lsb >> 4; // Bit shift to extract only the last 4 bits

        let h3 = self.read_byte(HumidityCalibrationCoefficients::H3 as u8)? as u32;
        let h4 = self.read_byte(HumidityCalibrationCoefficients::H4 as u8)? as u32;
        let h5 = self.read_byte(HumidityCalibrationCoefficients::H5 as u8)? as u32;
        let h6 = self.read_byte(HumidityCalibrationCoefficients::H6 as u8)? as u32;
        let h7 = self.read_byte(HumidityCalibrationCoefficients::H7 as u8)? as u32;

        // Look at datasheet p. 20 for explanation of the following calculations
        let var1 = (self.raw_humidity as i32) - ((h1 as i32) << 4) - (((self.temperature * (h3 as i32)) / 100) >> 1);
        let var2 = ((h2 as i32) * (((self.temperature * (h4 as i32)) / 100) + (((self.temperature * ((self.temperature * (h5 as i32)) / 100)) >> 6) / 100) + (  (1 << 14)))) >> 10;
        let var3 = var1 * var2;
        let var4 = (((h6 as i32) << 7) + ((self.temperature * (h7 as i32)) / 100)) >> 4;
        let var5 = ((var3 >> 14) * (var3 >> 14)) >> 10;
        let var6 = (var4 * var5) >> 1;
        let hum_comp = (((var3 + var6) >> 10) * 1000) >> 12;
        Ok(hum_comp)
    }

    fn read_byte(&mut self, register: u8) -> Result<u8, BmeError<T>> {
        let val = match self.i2c_handle.smbus_read_byte_data(register) {
            Ok(data) => Ok(data),
            Err(e) => Err(BmeError::I2CError(e)),
        };
        thread::sleep(Duration::from_millis(5));
        return val
    }

    fn write_byte(&mut self, register: u8, value: u8) -> Result<(), BmeError<T>> {
        let val = match self.i2c_handle.smbus_write_byte_data(register, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(BmeError::I2CError(e)),
        };
        thread::sleep(Duration::from_millis(5));
        return val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sensor_data() {
        let mut i2c_handle: LinuxI2CDevice = LinuxI2CDevice::new("/dev/i2c-1", BME680_I2C_ADDR).unwrap();
        let bme = match Bme680::new(i2c_handle) {
            Ok(bme) => bme,
            Err(e) => panic!("Error"),
        };
    }
}