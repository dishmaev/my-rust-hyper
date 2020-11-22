use super::super::{traits, entities};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct OnAsyncCommandStateChange {
    pub commands: Vec<entities::executor::AsyncCommandState>,
}

impl traits::ObjectType for OnAsyncCommandStateChange {
    fn get_type_name() -> &'static str {
        "OnAsyncCommandStateChange"
    }
}