use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub service_name: String,
    pub priority: i32,
    pub http_helth: String,
    pub mq_helth: String,
    pub command: Vec<Command>,
    pub subscription: Vec<Subscription>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    pub object_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mq_to: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    pub object_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mq_to: Option<String>,
}