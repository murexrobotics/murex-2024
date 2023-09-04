use std::error::Error;
use std::time::Duration;
use std::thread::sleep;

use rppal::uart::{Parity, Uart};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Connecting to UART...");
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    uart.set_read_mode(1, Duration::default())?;
    uart.set_write_mode(true)?;

    println!("Reading messages\n");
    let mut i: i64 = 1111111111111111;

    loop {
        let mut buffer = [0u8; 32];
        i += 1;

        // Write the Message
        uart.write(i.to_string().as_bytes()).unwrap();
        println!("<<< {:?}", i.to_string());

        // Listen to the response
        let n = uart.read(&mut buffer)?;
        println!("Read {n} bytes");
        if n > 0 {
            if let Ok(s) = String::from_utf8(buffer.to_vec()){
                println!(">>> {:?}", s);
            }
        }

        // Cap the speed of the program
        sleep(Duration::from_secs(1))
    }
}