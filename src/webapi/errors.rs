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
