use std::fmt;
use std::io::Error as IOError;
use std::{error::Error, net::SocketAddr};

use tokio::sync::broadcast::error::{RecvError, SendError};

#[derive(Debug)]
pub enum AsyncError {
    IoError(IOError),
    RecvError(RecvError),
    SendError(SendError<(String, SocketAddr)>),
}

// Implement `std::fmt::Display` for `MyError`
impl fmt::Display for AsyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsyncError::IoError(err) => write!(f, "IO error: {}", err),
            AsyncError::RecvError(recv_error) => write!(f, "RecvError {:?}", recv_error),
            AsyncError::SendError(send_error) => write!(f, "SendError {:?}", send_error),
        }
    }
}

// Implement `From` for `IO Error`
impl From<IOError> for AsyncError {
    fn from(err: IOError) -> Self {
        AsyncError::IoError(err)
    }
}

// Implement `From` for the RecvError
impl From<RecvError> for AsyncError {
    fn from(err: RecvError) -> Self {
        AsyncError::RecvError(err)
    }
}

// Implement `From` for the RecvError
impl From<SendError<(String, SocketAddr)>> for AsyncError {
    fn from(err: SendError<(String, SocketAddr)>) -> Self {
        AsyncError::SendError(err)
    }
}

impl Error for AsyncError {}
