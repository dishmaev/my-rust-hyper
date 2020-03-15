use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Copy, Clone)]
#[repr(i16)]
pub enum ErrorCode {
    ReplyOk = 0,
    ReplyErrorDatabase = -1,
    #[cfg(not(test))]
    ReplyErrorNotFound = -100,
}

impl ErrorCode {
    pub fn as_isize(self) -> isize { self as isize }
}
