use super::super::errors;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
}

impl Reply {
    pub fn is_ok(&self) -> bool {
        (self.error_code == errors::ErrorCode::ReplyOk)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddIntIdsReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i32>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddStrIdsReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

impl AddStrIdsReply {
    pub fn is_ok(&self) -> bool {
        (self.error_code == errors::ErrorCode::ReplyOk)
    }

    pub fn get_ids(&self) -> Vec<String> {
        self.ids.clone().unwrap()
    }
}
