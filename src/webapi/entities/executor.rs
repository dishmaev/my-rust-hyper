use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::EnumString;

#[derive(
    Deserialize, Serialize, Debug, PartialEq, Copy, Clone, EnumString, ToString, JsonSchema,
)]
pub enum CommandSystemState {
    Initial, //default for new async command
    Completed,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct SendedAsyncCommand {
    pub id: String,
    pub object_type: String,
    pub service_name: String,
    pub state: String,
    pub change_state_event: i32,
    pub added_at: DateTime<Utc>,
    pub state_changed_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<SendedAsyncCommandHistory>>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct SendedAsyncCommandHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    pub state: String,
    pub added_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ReceivedAsyncCommand {
    pub id: String, //from correlaton_id
    pub object_type: String,
    pub service_name: String,
    pub request_body: String,
    pub state: String,
    pub change_state_event: i32, //if = 1, must be send OnAsyncCommandStateChange for each state change, except system states
    pub reply_body: String,
    pub proto: String,
    pub added_at: DateTime<Utc>,
    pub state_changed_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<ReceivedAsyncCommandHistory>>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ReceivedAsyncCommandHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    pub state: String,
    pub added_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct AsyncCommandState {
    pub id: String, //from correlaton_id
    pub state: String,
    pub state_changed_at: DateTime<Utc>,
}
