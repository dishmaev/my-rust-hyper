use super::entities::route;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct AppSettings {
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