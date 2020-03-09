use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub id: Option<i32>,
    pub car_name: String,
}
