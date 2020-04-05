use sqlx::FromRow;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, Clone, FromRow, JsonSchema)]
pub struct Usr {
    pub id: i32,
    pub usr_name: String,
    pub usr_password: String,
}
