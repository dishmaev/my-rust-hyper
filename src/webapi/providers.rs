use super::{connectors, errors};
use hyper::{Body, Client, Request, Method, StatusCode};

pub struct HttpProvider;

impl HttpProvider {
    pub async fn new() -> connectors::Result<HttpProvider> {
        Ok(HttpProvider {})
    }

    pub async fn execute(
        &self,
        to: &str,
        object_type: &str,
        correlation_id: &str,
        bat: String,
        body: Body,
    ) -> connectors::Result<Body> {
        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}?ObjectType={}&CorrelationId={}", to, object_type, correlation_id))
            .header("Authorization", bat)
            .body(body)
            .expect("request builder");
        let client = Client::new();
        let resp = client.request(req).await?;
        debug!(
            "correlation id {} http response code {}",
            correlation_id,
            resp.status(),
        );
        let (parts, body) = resp.into_parts();
        if parts.status == StatusCode::OK {
            Ok(body)
        }
        else{
            Err(errors::ProtoProviderError.into())
        }
    }
}

pub struct MqProvider;

impl MqProvider {
    pub async fn new() -> connectors::Result<MqProvider> {
        Ok(MqProvider {})
    }

    pub async fn execute(
        &self,
        to: &str,
        object_type: &str,
        correlation_id: &str,
        bat: String,
        body: Body,
    ) -> connectors::Result<Body> {
        Ok(Body::empty())
    }
}
