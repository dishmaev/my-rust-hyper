use super::entities::route;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub _error: Option<HashMap<isize, String>>,
    pub _access: Option<Access>,
    pub _pg_db: PgDb,
    pub _my_sql_db: MySqlDb,
    pub _service: route::Route,
    pub _router: Router
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Router {
    pub http_from: Option<String>,
    pub mq_from: Option<String>,
    pub local: Option<route::Route>
}
