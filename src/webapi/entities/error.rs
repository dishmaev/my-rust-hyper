use serde::{Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub id: i32,
    pub error_name: String,
}