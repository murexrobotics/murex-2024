use core::fmt::{Debug, Display};
// use core::error::Error;
// TODO: Use nightly rust to implement core `Error` trait

pub type Result<T, E> = core::result::Result<T, Error<E>>;

#[derive(Debug)]
pub enum Error<E: Debug> {
    I2CError(E),
    ConfigError,
}

impl<E: Debug> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::I2CError(e)
    }
}

impl<E: Debug> Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::I2CError(e) => write!(f, "I2C Error: {:?}", e),
            Error::ConfigError => write!(f, "Config Error"),
        }
    }
}
