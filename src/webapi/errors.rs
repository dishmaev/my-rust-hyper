use serde_repr::Serialize_repr;

#[derive(Serialize_repr, Debug)]
#[repr(i16)]
pub enum ErrorCode {
    ReplyOk = 0,
    ReplyErrorDatabase = -1,
    ReplyErrorNotFound = -100,
}
