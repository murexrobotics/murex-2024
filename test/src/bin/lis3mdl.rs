use embedded_hal::blocking::delay::DelayMs;
use linux_embedded_hal;
use linux_embedded_hal::{Delay, I2cdev};
use lis3mdl_driver::{Address, Lis3mdl};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").expect("Failed to initialize I2C bus");
    let mut lis = Lis3mdl::new(i2c, Address::Addr1C).expect("Failed to initialize LIS3MDL");

    let mut delay = Delay {};

    loop {
        let mag = lis.get_mag_axes_mgauss().expect("Failed to read lis data");
        println!("{:?}", mag);
        delay.delay_ms(1000u32);
    }
}
