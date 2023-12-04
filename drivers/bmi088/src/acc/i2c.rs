#![allow(unreachable_code)]

use super::config::Config;
use super::consts::*;
use super::error::Result;
use embedded_hal::blocking::i2c::{
    Operation, Read, Transactional, TransactionalIter, Write, WriteRead,
};

pub struct Acc<I2C, E>
where
    I2C: WriteRead<Error = E> + Write<Error = E> + Read<Error = E>,
    I2C: Transactional<Error = E> + TransactionalIter<Error = E>,
    E: core::fmt::Debug,
{
    i2c_handle: I2C,
    address: u8,
    config: Config,
}

impl<I2C, E> Acc<I2C, E>
where
    I2C: WriteRead<Error = E> + Write<Error = E> + Read<Error = E>,
    I2C: Transactional<Error = E> + TransactionalIter<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c_handle: I2C) -> Self {
        let mut acc = Self {
            i2c_handle,
            address: 0x64, // TODO: Change to real address
            config: Config::default(),
        };
        acc.init_accelerometer();
        acc
    }

    fn init_accelerometer(&mut self) -> Result<(), E> {
        // TODO: Go through and add logging
        // TODO: Conditional enabling of configs based on other configs, eg int1.enabled makes other possible
        // TODO: Mark mutually exclusive configs as such

        let mut operations = vec![];

        // Acc Basic Config
        if self.config.enabled {
            operations.push(Operation::Write(&[ACC_PWR_CTRL, ACC_ON]))
        }
        if self.config.active_mode {
            operations.push(Operation::Write(&[ACC_PWR_CONF, ACC_ACTIVE_MODE]))
        }

        // Acc Interupt Config
        let mut int1_conf: u8 = 0;
        if self.config.int1.input {
            int1_conf |= INT1_INPUT_EN
        }
        if self.config.int1.output {
            int1_conf |= INT1_OUTPUT_EN
        }
        if !self.config.int1.push_pull {
            int1_conf |= INT1_OPEN_DRAIN
        }
        if self.config.int1.active_high {
            int1_conf |= INT1_ACTIVE_HIGH
        }

        let int1_conf_buf = [INT1_IO_CTRL, int1_conf];
        if int1_conf != 0 {
            operations.push(Operation::Write(&int1_conf_buf))
        }

        let mut int2_conf: u8 = 0;
        if self.config.int2.input {
            int2_conf |= INT2_INPUT_EN
        }
        if self.config.int2.output {
            int2_conf |= INT2_OUTPUT_EN
        }
        if !self.config.int2.push_pull {
            int2_conf |= INT2_OPEN_DRAIN
        }
        if self.config.int2.active_high {
            int2_conf |= INT2_ACTIVE_HIGH
        }

        let int2_conf_buf = [INT1_IO_CTRL, int2_conf];
        if int2_conf != 0 {
            operations.push(Operation::Write(&int2_conf_buf))
        }

        let mut int_map: u8 = 0;
        if self.config.int1.map_data_ready {
            int_map |= INT1_DATA_READY_MAPPING
        }
        if self.config.int1.map_fifo_watermark {
            int_map |= INT1_FIFO_WATERMARK_INT_MAPPING
        }
        if self.config.int1.map_fifo_full {
            int_map |= INT1_FIFO_FULL_INT_MAPPING
        }
        if self.config.int2.map_data_ready {
            int_map |= INT2_DATA_READY_MAPPING
        }
        if self.config.int2.map_fifo_watermark {
            int_map |= INT2_FIFO_WATERMARK_INT_MAPPING
        }
        if self.config.int2.map_fifo_full {
            int_map |= INT2_FIFO_FULL_INT_MAPPING
        }

        let int_map_buf = [INT_MAP_DATA, int_map];
        if int_map != 0 {
            operations.push(Operation::Write(&int_map_buf))
        }

        // Acc FIFO Config
        let mut fifo_conf: u8 = 0;
        if self.config.fifo_enabled {
            fifo_conf |= ENABLE_ACC_FIFO
        }
        if self.config.int1.store_int_data {
            fifo_conf |= ENABLE_FIFO_INT1_STORE
        }
        if self.config.int2.store_int_data {
            fifo_conf |= ENABLE_FIFO_INT2_STORE
        }

        let fifo_conf_buf = [FIFO_CONFIG_1, fifo_conf];
        if fifo_conf != 0 {
            operations.push(Operation::Write(&fifo_conf_buf))
        }

        if !self.config.int1.stream_mode {
            operations.push(Operation::Write(&[FIFO_CONFIG_0, ACC_FIFO_MODE]))
        }

        // Acc Self Test
        if let Some(self_test) = self.config.self_test {
            if self_test {
                operations.push(Operation::Write(&[ACC_SELF_TEST, ACC_TEST_POSITIVE]))
            } else {
                operations.push(Operation::Write(&[ACC_SELF_TEST, ACC_TEST_OFF]))
            }
        }

        self.i2c_handle
            .exec_iter(self.address, operations.into_iter())?;
        Ok(())
    }

    pub fn restart(&mut self) -> Result<(), E> {
        todo!();
        self.i2c_handle
            .write(self.address, &[ACC_SOFTRESET, ACC_SOFTRESET_CMD])?;
        // TODO: Add delay
        if self.config.config_after_restart {
            self.init_accelerometer()?;
        }
        Ok(())
    }

    pub fn set_config(&mut self, config: Config) -> Result<(), E> {
        self.config = config;
        Ok(())
    }

    pub fn get_sensor_time() -> i32 {
        unimplemented!()
    }

    pub fn get_sensor_temp() -> i16 {
        unimplemented!()
    }

    pub fn get_accelerometer_data() -> (i16, i16, i16) {
        todo!()
    }
}
