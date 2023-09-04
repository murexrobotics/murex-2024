use std::error::Error;
use std::time::Duration;
use rppal::uart::{Parity, Uart};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Connecting to UART...");
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    uart.set_read_mode(1, Duration::default())?;
    uart.set_write_mode(true)?;
    uart.write("Hello World".as_bytes())?;
    Ok(())
}