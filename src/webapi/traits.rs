// use super::connectors;
// use hyper::Body;
// use std::collections::HashMap;

pub trait ObjectType {
    fn get_type_name() -> &'static str;
}

/*pub trait CommandState {
    fn get_states() -> &'static HashMap<&'static str, &'static str>; // state/description
}*/

/* async fn not supported yet
pub trait Provider {
    async fn execute(
        &self,
        to: &str,
        object_type: &str,
        correlation_id: &str,
        bat: String,
        body: Body,
    ) -> connectors::Result<Body>;
}
*/