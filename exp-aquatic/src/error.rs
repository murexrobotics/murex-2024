use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    UdpSocketError,
    UdpSocketTimeout,
    UdpSocketSendError,
    UdpSocketRecvError,
    ConnectionInitializationTimeout,
    ConnectionHandshakeFailed
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UdpSocketError => write!(f, "UdpSocketError"),
            Error::UdpSocketTimeout => write!(f, "UdpSocketTimeout"),
            Error::UdpSocketSendError => write!(f, "UdpSocketSendError"),
            Error::UdpSocketRecvError => write!(f, "UdpSocketRecvError"),
            Error::ConnectionInitializationTimeout => write!(f, "ConnectionInitializationTimeout"),
            Error::ConnectionHandshakeFailed => write!(f, "ConnectionHandshakeFailed"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::UdpSocketError
    }
}