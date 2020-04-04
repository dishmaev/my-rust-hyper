use super::super::{entities::route, errors, traits};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetRouteReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<route::Route>>,
}

impl traits::ObjectType for GetRouteReply {
    fn get_type_name() -> &'static str {
        "GetRouteReply"
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetServiceCommandReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<route::Command>>,
}

impl traits::ObjectType for GetServiceCommandReply {
    fn get_type_name() -> &'static str {
        "GetServiceCommandReply"
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetServiceEventReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<route::Event>>,
}

impl traits::ObjectType for GetServiceEventReply {
    fn get_type_name() -> &'static str {
        "GetServiceEventReply"
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetServiceSubscriptionReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<route::Subscription>>,
}

impl traits::ObjectType for GetServiceSubscriptionReply {
    fn get_type_name() -> &'static str {
        "GetServiceSubscriptionReply"
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetServiceReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<route::Service>>,
}

impl traits::ObjectType for GetServiceReply {
    fn get_type_name() -> &'static str {
        "GetServiceReply"
    }
}
