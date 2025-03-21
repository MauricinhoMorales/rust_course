use std::any::Any;
use std::error::Error;
use std::fmt;
use std::sync::mpsc::{RecvError, SendError};

#[derive(Debug)]
pub enum ThreadError {
    SendError(SendError<String>),
    ThreadPanic(Box<dyn Any + Send + 'static>),
    RecvError(RecvError),
}

// Implement `std::fmt::Display` for `MyError`
impl fmt::Display for ThreadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadError::SendError(err) => write!(f, "Send error: {}", err),
            ThreadError::ThreadPanic(error) => write!(f, "Thread panicked{:?}", error),
            ThreadError::RecvError(recv_error) => write!(f, "Received error {:?}", recv_error),
        }
    }
}

// Implement `From` for `SendError<String>`
impl From<SendError<String>> for ThreadError {
    fn from(err: SendError<String>) -> Self {
        ThreadError::SendError(err)
    }
}

// Implement `From` for the thread's panic payload
impl From<Box<dyn Any + Send + 'static>> for ThreadError {
    fn from(err: Box<dyn Any + Send + 'static>) -> Self {
        ThreadError::ThreadPanic(err)
    }
}

// Implement `From` for the RecvError
impl From<RecvError> for ThreadError {
    fn from(err: RecvError) -> Self {
        ThreadError::RecvError(err)
    }
}

impl Error for ThreadError {}
