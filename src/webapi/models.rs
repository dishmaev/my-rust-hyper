use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub connection_string: String,
    pub authentication: HashMap<String, String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub id: i32,
    pub error_name: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Usr {
    pub id: i32,
    pub usr_name: String,
    pub usr_password: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub id: Option<i32>,
    pub car_name: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,

    pub call_back: String
}