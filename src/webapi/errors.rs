use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone, ToString, JsonSchema)]
pub enum ErrorCode {
    ReplyOk,
    FutureOk, //async command call result, use only with http proto
    TooManyRequestsError, //if async queue or reply queue full, http proto return HTTP-status 429
    DatabaseError,
    #[cfg(not(test))]
    NotFoundError,
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
pub struct UnknownServiceNameError;

impl fmt::Display for UnknownServiceNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown service name error")
    }
}

impl error::Error for UnknownServiceNameError {
    fn description(&self) -> &str {
        "unknown service name error"
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
pub struct BadReplyCommandError;

impl fmt::Display for BadReplyCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bad reply command error")
    }
}

impl error::Error for BadReplyCommandError {
    fn description(&self) -> &str {
        "bad reply command error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct CallCommandError;

impl fmt::Display for CallCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "call command error")
    }
}

impl error::Error for CallCommandError {
    fn description(&self) -> &str {
        "call command error"
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
pub struct SendEventError;

impl fmt::Display for SendEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "send event error")
    }
}

impl error::Error for SendEventError {
    fn description(&self) -> &str {
        "send event error"
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
