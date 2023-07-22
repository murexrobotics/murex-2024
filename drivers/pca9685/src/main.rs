use linux_embedded_hal::I2cdev;
use pca9685::PCA9685;
use std::{
    thread::sleep,
    time::Duration
};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut pca = PCA9685::new(dev);
    println!("Setting PWM frequency to 50Hz and restarting");
    pca.set_pwm_freq(50).unwrap();
    // pca.restart().unwrap();

    println!("Duty Cycle = 205");
    pca.set_duty_cycle(14, 205).unwrap();
    sleep(Duration::from_secs(2));

    println!("Duty Cycle = 310");
    pca.set_duty_cycle(14, 310).unwrap();
    sleep(Duration::from_secs(2));

    println!("Duty Cycle = 400");
    pca.set_duty_cycle(14, 400).unwrap();
    sleep(Duration::from_secs(2));

    println!("Duty Cycle = 1000");
    pca.set_duty_cycle(14, 1000).unwrap();
    sleep(Duration::from_secs(2));
}