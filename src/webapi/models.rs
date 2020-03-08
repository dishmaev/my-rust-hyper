use super::errors;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i32>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub _access: Option<Access>,
    pub _pg_db: PgDb,
    pub _my_sql_db: MySqlDb,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Access {
    pub authentication: HashMap<String, String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PgDb {
    pub connection_string: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MySqlDb {
    pub connection_string: String,
}