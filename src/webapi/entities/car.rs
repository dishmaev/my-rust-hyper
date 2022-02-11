use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Clone, FromRow, JsonSchema)]
pub struct Car {
    pub id: Option<i32>,
    pub car_name: String,
}
