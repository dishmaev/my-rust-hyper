use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub connection_string: String,
    pub authentication: Vec<UserPassword>,
    pub error: Vec<Error>,
}

#[derive(Deserialize)]
pub struct UserPassword {
    pub user: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Error {
    pub code: isize,
    pub name: String
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub error_code: isize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddReply {
    pub error_code: isize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i32>>
}
