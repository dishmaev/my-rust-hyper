use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub id: Option<i32>,
    //    timestamp: f64,
    //    kind: String,
    //    tags: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Command {
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub authentication: Vec<UserPassword>,
}

#[derive(Deserialize)]
pub struct UserPassword {
    pub user: String,
    pub password: String,
}
