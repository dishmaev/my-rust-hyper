use super::super::traits;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct OnServiceUnavailable {
    pub services: Vec<String>,
}

impl traits::ObjectType for OnServiceUnavailable {
    fn get_type_name() -> &'static str {
        "OnServiceUnavailable"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct OnRouteUpdate {
    pub services: Vec<String>,
}

impl traits::ObjectType for OnRouteUpdate {
    fn get_type_name() -> &'static str {
        "OnRouteUpdate"
    }
}
