use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone, ToString, JsonSchema)]
pub enum ErrorCode {
    ReplyOk,                  //command success result
    AsyncOk,                  //async command success registered
    UnsupportedExecModeError, //if not support sync/async call mode
    TooManyRequestsError, //if async queue or reply queue full, http proto return HTTP-status 429
    TimeoutExpiredError,
    DatabaseError, //internal error, database level
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
pub struct UnsupportedProtoError;

impl fmt::Display for UnsupportedProtoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unsupported proto error")
    }
}

impl error::Error for UnsupportedProtoError {
    fn description(&self) -> &str {
        "supported proto error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct ProtoProviderError;

impl fmt::Display for ProtoProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "proto provider error")
    }
}

impl error::Error for ProtoProviderError {
    fn description(&self) -> &str {
        "proto provider error"
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
pub struct GeRemoteRouterError;

impl fmt::Display for GeRemoteRouterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "get from remote router error")
    }
}

impl error::Error for GeRemoteRouterError {
    fn description(&self) -> &str {
        "get from remote router error"
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

#[derive(Debug, Clone)]
pub struct TooManyRequestsError;

impl fmt::Display for TooManyRequestsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "too many request error")
    }
}

impl error::Error for TooManyRequestsError {
    fn description(&self) -> &str {
        "too many request error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct TimeoutExpiredError;

impl fmt::Display for TimeoutExpiredError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "timeout expired error")
    }
}

impl error::Error for TimeoutExpiredError {
    fn description(&self) -> &str {
        "timeout expired error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct UnknownErrorCodeError;

impl fmt::Display for UnknownErrorCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown error code error")
    }
}

impl error::Error for UnknownErrorCodeError {
    fn description(&self) -> &str {
        "unknown error code error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct AsyncCommandNotFoundError;

impl fmt::Display for AsyncCommandNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "async command not found error")
    }
}

impl error::Error for AsyncCommandNotFoundError {
    fn description(&self) -> &str {
        "async command not found error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct DataIntegrityError;

impl fmt::Display for DataIntegrityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "data integrity error")
    }
}

impl error::Error for DataIntegrityError {
    fn description(&self) -> &str {
        "data integrity error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct UnsupportedExecModeError;

impl fmt::Display for UnsupportedExecModeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unsupported exec mode error")
    }
}

impl error::Error for UnsupportedExecModeError {
    fn description(&self) -> &str {
        "unsupported exec mode error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct UnknownAsyncCommandStateError;

impl fmt::Display for UnknownAsyncCommandStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown async command state error")
    }
}

impl error::Error for UnknownAsyncCommandStateError {
    fn description(&self) -> &str {
        "unknown async command state error"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
