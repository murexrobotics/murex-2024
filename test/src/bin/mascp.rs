use rppal::uart::{Parity, Uart};
use std::error::Error;
use std::time::Duration;
use std::thread::sleep;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Connecting to UART...");
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    uart.set_read_mode(1, Duration::default())?;
    uart.set_write_mode(true)?;

    // ------------ ESC all call ------------
    uart.write(&[48, 137])?;
    sleep(Duration::from_secs(1));

    uart.write(&[48, 127])?; 
    sleep(Duration::from_secs(1));

    // ------- Spin Individual Thsuters ------
    uart.write(&[49, 137])?; // Spind thruster 1
    sleep(Duration::from_secs(1));

    uart.write(&[50, 137])?; // Spind thruster 2
    sleep(Duration::from_secs(1));

    uart.write(&[51, 137])?; // Spind thruster 3
    sleep(Duration::from_secs(1));

    uart.write(&[52, 137])?; // Spind thruster 4
    sleep(Duration::from_secs(1));

    uart.write(&[53, 137])?; // Spind thruster 5
    sleep(Duration::from_secs(1));

    uart.write(&[54, 137])?; // Spind thruster 6
    sleep(Duration::from_secs(1));

    // ------------ ESC all call ------------
    uart.write(&[48, 127])?;
    sleep(Duration::from_secs(1));

    Ok(())
}
