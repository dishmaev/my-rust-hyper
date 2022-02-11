use schemars::JsonSchema;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Clone, FromRow, JsonSchema)]
pub struct Usr {
    pub id: i32,
    pub usr_name: String,
    pub usr_password: String,
}
