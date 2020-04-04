use super::super::{entities::route, traits};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct GetRoute {
    pub filter: Option<String>,
    pub ids: Option<Vec<String>>,
}

impl traits::ObjectType for GetRoute {
    fn get_type_name() -> &'static str {
        "GetRoute"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct AddRoute {
    pub items: Vec<route::Route>,
}

impl traits::ObjectType for AddRoute {
    fn get_type_name() -> &'static str {
        "AddRoute"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct RemoveRoute {
    pub ids: Vec<String>,
}

impl traits::ObjectType for RemoveRoute {
    fn get_type_name() -> &'static str {
        "RemoveRoute"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct GetServiceCommand {
    pub filter: Option<String>,
    pub services: Option<Vec<String>>,
}

impl traits::ObjectType for GetServiceCommand {
    fn get_type_name() -> &'static str {
        "GetServiceCommand"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct GetServiceEvent {
    pub filter: Option<String>,
    pub services: Option<Vec<String>>,
}

impl traits::ObjectType for GetServiceEvent {
    fn get_type_name() -> &'static str {
        "GetServiceEvent"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct GetServiceSubscription {
    pub filter: Option<String>,
    pub services: Option<Vec<String>>,
}

impl traits::ObjectType for GetServiceSubscription {
    fn get_type_name() -> &'static str {
        "GetServiceSubscription"
    }
}

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct GetService {
    pub filter: Option<String>,
    pub ids: Option<Vec<String>>,
}

impl traits::ObjectType for GetService {
    fn get_type_name() -> &'static str {
        "GetService"
    }
}
