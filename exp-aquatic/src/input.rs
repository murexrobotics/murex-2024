use crate::actions::Action;
use std::net::UdpSocket;
use std::time::Duration;

const CONNECTION_ATTEMPTS: i32 = 5;
const PI_PORT: u16 = 5678;
const PI_IP_ADDRESS: &str = "192.168.100.1";
const MAX_MESSAGE_SIZE: usize = 16;

/// Initialize socket connection between the Pi and the topside computer.
/// Must receive 0xFF from topside computer to establish connection. 
pub fn initialize_connection() -> Option<UdpSocket> {
    // TODO: Switch return type to Result<UdpSocket> instead
    let mut attempts = 0;
    while attempts != CONNECTION_ATTEMPTS {
        attempts += 1;
        let socket = match UdpSocket::bind(format!("{}:{}", PI_IP_ADDRESS, PI_PORT)) {
            Ok(s) => s,
            Err(_) => continue,
        };

        socket.set_read_timeout(Some(Duration::from_secs(10)));

        let mut buf = [0; 1];
        let (len, src) = socket.recv_from(&mut buf).ok()?; // TODO: .ok()? is suboptimal solution, should be replaced in future.

        if len == 1 && buf[0] == 0xFF {
            socket.connect(src).ok()?;
            socket.send(&[0xFF]).ok()?;
            return Some(socket)
        } else {
            return None // Handshake failed
        }
    }
    None // Exceeded connection attempts
}

pub fn receive(socket: &UdpSocket) -> Box<dyn Action> {
    let mut buf = [0; MAX_MESSAGE_SIZE];
    socket.recv(&mut buf).unwrap(); // TODO: Add log message to say that something went wrong with the socket
    decode_input(&buf)
}

/// Decode Controller Input
fn decode_input(input: &[u8]) -> Box<dyn Action> {
    // Conventions for decoding will be decided in fall when we have controller
    todo!();
}

