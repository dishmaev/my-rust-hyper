use super::super::{errors, traits};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StandardReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
}

impl StandardReply {
    pub fn is_ok(&self) -> bool {
        self.error_code == errors::ErrorCode::ReplyOk
    }
}

impl traits::ObjectType for StandardReply {
    fn get_type_name() -> &'static str {
        "StandardReply"
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AddIntIdsReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i32>>,
}

impl traits::ObjectType for AddIntIdsReply {
    fn get_type_name() -> &'static str {
        "AddIntIdsReply"
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AddStrIdsReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

impl AddStrIdsReply {
    pub fn is_ok(&self) -> bool {
        self.error_code == errors::ErrorCode::ReplyOk
    }
}

impl traits::ObjectType for AddStrIdsReply {
    fn get_type_name() -> &'static str {
        "AddStrIdsReply"
    }
}
