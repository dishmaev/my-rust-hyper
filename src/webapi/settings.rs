use super::entities::route;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct AppSettings {
    pub max_async_command_queue_length: u16,
    pub max_async_command_reply_wait_in_hours: u16,
    pub max_sync_command_reply_wait_in_seconds: u16,
    pub error: Option<HashMap<String, String>>,
    pub access: Access,
    pub database: HashMap<String, Database>,
    pub router: Option<HashMap<String, String>>,
    pub path: HashMap<String, route::ServicePath>,
    pub service: HashMap<String, route::Route>,
}

#[derive(Deserialize)]
pub struct Access {
    pub authentication: Authentication,
}

#[derive(Deserialize)]
pub struct Authentication {
    pub server: HashMap<String, String>,
    pub client: Vec<route::ClientAccess>
}

#[derive(Deserialize)]
pub struct Database {
    pub connection_string: String,
}