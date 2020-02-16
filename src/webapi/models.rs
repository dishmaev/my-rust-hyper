use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub id: Option<i32>,
    //    timestamp: f64,
    //    kind: String,
    //    tags: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Command{
    pub name: Option<String>
}
