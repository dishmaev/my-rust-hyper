use schemars::JsonSchema;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct SendedAsyncCommand {
    pub id: String,
    pub object_type: String,
    pub proto: String,
    pub state_to: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct ReceivedAsyncCommand {
    pub id: String,
    pub object_type: String,
    pub proto: String,
    pub reply_to: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
