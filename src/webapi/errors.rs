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
pub struct ChannelError;

impl fmt::Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "channel error")
    }
}

impl error::Error for ChannelError {
    fn description(&self) -> &str {
        "channel error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct SignalSendError;

impl fmt::Display for SignalSendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "signal send error")
    }
}

impl error::Error for SignalSendError {
    fn description(&self) -> &str {
        "signal send error"
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

#[derive(Debug, Clone)]
pub struct UnknownCommandError;

impl fmt::Display for UnknownCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown command error")
    }
}

impl error::Error for UnknownCommandError {
    fn description(&self) -> &str {
        "unknown command error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct UnknownEventError;

impl fmt::Display for UnknownEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown event error")
    }
}

impl error::Error for UnknownEventError {
    fn description(&self) -> &str {
        "unknown event error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct UnsetRequiredValueError;

impl fmt::Display for UnsetRequiredValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "required value not set")
    }
}

impl error::Error for UnsetRequiredValueError {
    fn description(&self) -> &str {
        "required value not set"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
