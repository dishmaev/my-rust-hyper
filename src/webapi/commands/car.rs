use super::super::{entities::car, traits};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct GetCar {
    pub filter: Option<String>,
    pub ids: Option<Vec<i32>>,
}

impl traits::ObjectType for GetCar {
    fn get_type_name() -> &'static str {
        "GetCar"
    }
}

// impl traits::CommandState for GetCar {
//     fn get_states() -> &'static HashMap<&'static str, &'static str> {
//         [("one", "one"), ("two", "two")]
//     }
// }

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct AddCar {
    pub items: Vec<car::Car>,
}

impl traits::ObjectType for AddCar {
    fn get_type_name() -> &'static str {
        "AddCar"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct ChangeCar {
    pub items: Vec<car::Car>,
}

impl traits::ObjectType for ChangeCar {
    fn get_type_name() -> &'static str {
        "ChangeCar"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct RemoveCar {
    pub ids: Vec<i32>,
}

impl traits::ObjectType for RemoveCar {
    fn get_type_name() -> &'static str {
        "RemoveCar"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct ReserveCar {
    pub services: Vec<i32>,
}

impl traits::ObjectType for ReserveCar {
    fn get_type_name() -> &'static str {
        "ReserveCar"
    }
}
