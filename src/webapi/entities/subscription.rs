use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,

    pub call_back: String,
}
