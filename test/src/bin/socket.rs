use std::error::Error;
use std::net::UdpSocket;
use std::time::Duration;

const PI_PORT: u16 = 5678;
const PI_IP_ADDRESS: &str = "192.168.100.1";

fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(format!("{}:{}", PI_IP_ADDRESS, PI_PORT))?;

    socket.set_read_timeout(Some(Duration::from_secs(10)))?;

    let mut buf = [0; 2];

    let (len, src) = socket.recv_from(&mut buf)?;

    if len == 1 && buf[0] == 0xFF {
        socket.connect(src)?;

        socket.send(&[0xFF])?;
    }

    Ok(())
}
