use std::error::Error;
use std::net::UdpSocket;
// use std::time::Duration;

const PI_PORT: u16 = 5678;
const PI_IP_ADDRESS: &str = "192.168.100.1";

fn main() -> Result<(), Box<dyn Error>> {
    println!("Started");
    let socket = UdpSocket::bind(format!("{}:{}", PI_IP_ADDRESS, PI_PORT))?;
    println!("Socket bound");
    // socket.set_read_timeout(Some(Duration::from_secs(10)))?;

    let mut buf = [0; 2];
    let (len, src) = socket.recv_from(&mut buf)?;
    println!("Received {} bytes from {:?}", len, src);
    println!("buf: {:?}", buf);

    socket.send_to(&buf, "192.168.100.54:5678")?;

    // socket.send(&[buf[0]+1, buf[1]])?;
    println!("Sent");

    if len == 1 && buf[0] == 48 {
        println!("Inside special case");
        socket.connect(src)?;
        socket.send_to(&buf, "192.168.100.54:5678")?;
        socket.send_to(&[48], "192.168.100.54:5678")?;
    }

    Ok(())
}
