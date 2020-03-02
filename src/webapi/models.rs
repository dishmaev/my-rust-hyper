use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub authentication: HashMap<String, String>,
    pub pgDb: PgDb,
    pub mySqlDb: MySqlDb,
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub id: i32,
    pub error_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Usr {
    pub id: i32,
    pub usr_name: String,
    pub usr_password: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub id: Option<i32>,
    pub car_name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,

    pub call_back: String,
}
