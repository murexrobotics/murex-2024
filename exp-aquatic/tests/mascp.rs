use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use rppal::uart::{Parity, Uart};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Connecting to UART...");
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    uart.set_read_mode(1, Duration::default())?;
    uart.set_write_mode(true)?;

    let messages = [
        [0x00],
        [0x01],
        [0x02],
        [0x03],
        [0x04],
        [0x05],
        [0x06],
        [0x07],
        [0x08],
        [0x09],
    ];

    println!("Reading messages\n");
    for msg in messages {
        let mut buffer = [0u8; 16];
        println!("<<< {msg:?}");

        uart.write(&msg)?;

        println!("Reading Now");
        uart.read(&mut buffer)?;
        println!("{:?}", String::from_utf8(buffer.to_vec()).unwrap());

        sleep(Duration::from_secs(1));
    }
    Ok(())
}
