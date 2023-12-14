use std::net::UdpSocket;
use std::time::Duration;
use std::error::Error;

use rppal::uart::{Parity, Uart};

const PI_PORT: u16 = 5678;
const PI_IP_ADDRESS: &str = "192.168.100.1";

fn main() -> Result<(), Box<dyn Error>> {
    println!("Started");
    let socket = UdpSocket::bind(format!("{}:{}", PI_IP_ADDRESS, PI_PORT))?;

    println!("Socket bound");
    // socket.set_read_timeout(Some(Duration::from_secs(10)))?;
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    uart.set_read_mode(1, Duration::default())?;
    uart.set_write_mode(true)?;

    let mut buf = [0; 2];
    let (len, src) = socket.recv_from(&mut buf)?;

    println!("Received {} bytes from {:?}", len, src);
    println!("buf: {:?}", buf);
    if len == 1 {
        socket.connect(src)?;
        socket.send(&[48])?;
    }

    loop {
        println!("Waiting for data");
        let len = socket.recv(&mut buf)?;
        println!("buf: {:?}", buf);

        if len == 2 && buf[0] == 48 {
            println!("Sending data");
            uart.write(&[48, buf[1].clone()])?;
        }
    }
}
