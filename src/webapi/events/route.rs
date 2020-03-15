use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnServiceUnavailable {
    pub service_name: String
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnRouteCommandUpdate {
    pub service_name: String
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnRouteSubscriptionUpdate {
    pub service_name: String,
}
