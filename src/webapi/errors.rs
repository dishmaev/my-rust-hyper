use serde_repr::{Deserialize_repr, Serialize_repr};
use std::error;
use std::fmt;

#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(i16)]
pub enum ErrorCode {
    ReplyOk = 0,
    ReplyErrorDatabase = -1,
    #[cfg(not(test))]
    ReplyErrorNotFound = -100,
}

impl ErrorCode {
    pub fn as_isize(self) -> isize {
        self as isize
    }
}

#[derive(Debug, Clone)]
pub struct ChannelTerminate;

impl fmt::Display for ChannelTerminate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "channel terminate")
    }
}

impl error::Error for ChannelTerminate {
    fn description(&self) -> &str {
        "channel terminate"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct EventSendError;

impl fmt::Display for EventSendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "event send error")
    }
}

impl error::Error for EventSendError {
    fn description(&self) -> &str {
        "event send error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct HandlerError;

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "handler error")
    }
}

impl error::Error for HandlerError {
    fn description(&self) -> &str {
        "handler error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
