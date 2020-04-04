use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    pub error_code: String,
    pub error_name: String,
}
