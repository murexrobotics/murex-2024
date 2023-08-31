use std::net::UdpSocket;
use crate::error::{Error, Result};
use crate::actions::Actions;
use std::time::Duration;

// use std::sync::mpsc::channel;
// use std::thread::Thread;

const PI_PORT: u16 = 1234;
const PI_IP_ADDRESS: &str = "";
const MAX_MESSAGE_SIZE: usize = 16;

/// Initialize socket connection between the Pi and the topside computer.
/// Must receive 0xFF from topside computer to establish connection.
pub fn initialize_connection() -> Result<UdpSocket> {
    let socket = UdpSocket::bind(format!("{}:{}", PI_IP_ADDRESS, PI_PORT)).map_err(Error::from)?;
    socket.set_read_timeout(Some(Duration::from_secs(10)));

    let mut buf = [0; 1];
    let (len, src) = socket.recv_from(&mut buf).map_err(Error::from)?;

    if len == 1 && buf[0] == 0xFF {
        socket.connect(src).map_err(Error::from)?;
        socket.send(&[0xFF]).map_err(Error::from)?;
        Ok(socket)
    } else {
        Err(Error::ConnectionHandshakeFailed)
    }
}

pub fn receive(socket: &UdpSocket) -> Result<Vec<Actions>> {
    let mut buf = [0; MAX_MESSAGE_SIZE];
    socket.recv(&mut buf).map_err(Error::from)?;
    decode_input(&buf)
}

/// Decode Controller Input
fn decode_input(input: &[u8]) -> Result<Vec<Actions>> {
    // Conventions for decoding will be decided in fall when we have controller
    todo!();
}

