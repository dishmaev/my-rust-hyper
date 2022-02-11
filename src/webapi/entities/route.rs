use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone, ToString, JsonSchema)]
pub enum ServiceState {
    Alive,
    Unavailable,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct Route {
    pub service_name: Option<String>,
    pub description: String,
    pub priority: i32,
    pub command: Vec<ServiceCommand>,
    pub event: Vec<ServiceEvent>,
    pub subscription: Vec<ServiceSubscription>,
    pub path: Option<HashMap<String, ServicePath>>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ServicePath {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,
    pub helth: String,    //for router monitor
    pub schema: String,   //for router helper
    pub reply_to: String, //for create reply_to path
    pub state: String,    //for get async command state
    pub error: String,    //for get error description path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>, //for create command call path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>, //for create subscription call path
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub state: String,
    pub added_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ServiceHelth {
    pub state: ServiceState,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ServiceCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    pub object_type: String,
    pub description: String,
    pub reply_type: String,
    pub exec_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ServiceEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    pub object_type: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ServiceSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    pub object_type: String,
    pub path: Option<HashMap<String, String>>,
}

#[derive(Clone)]
pub struct CommandRoute {
    pub service_name: Option<String>,
    pub object_type: String,
    pub reply_type: String,
    pub exec_mode: String,
    pub state: HashMap<String, String>, // state/description
    pub path: HashMap<String, String>,  // proto/to
}

#[derive(Clone)]
pub struct SubscriptionRoute {
    pub service_name: Option<String>,
    pub object_type: String,
    pub path: HashMap<String, String>, // proto/to
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ClientAccess {
    pub service_name: String,
    pub usr_name: String,
    pub usr_password: String,
}
