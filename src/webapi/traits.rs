// use super::connectors;
// use hyper::Body;

pub trait ObjectType {
    fn get_type_name() -> &'static str;
}

/* not supported yet
pub trait Provider {
    async fn execute(
        &self,
        to: &str,
        object_type: &str,
        correlation_id: &str,
        bat: String,
        body: Body,
    ) -> connectors::Result<Body>;
}*/