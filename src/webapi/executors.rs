use super::{access, connectors, errors, router, traits, workers};
use bytes::buf::BufExt;
use hyper::{Body, Client, Request, Response, Method, StatusCode};
use serde::{de, ser};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct CommandExecutor {
    ac: Arc<access::AccessChecker>,
    rt: Arc<router::Router>,
    cs: mpsc::Sender<workers::SignalCode>,
    hcp: HttpCommandProducer,
    mcp: MqCommandProducer,
}

impl CommandExecutor {
    pub async fn new(
        ac: Arc<access::AccessChecker>,
        rt: Arc<router::Router>,
        cs: mpsc::Sender<workers::SignalCode>,
    ) -> connectors::Result<CommandExecutor> {
        Ok(CommandExecutor {
            ac: ac,
            rt: rt,
            cs: cs,
            hcp: HttpCommandProducer::new().await?,
            mcp: MqCommandProducer::new().await?,
        })
    }

    pub async fn send_signal(&self, signal_code: workers::SignalCode) -> connectors::Result<()> {
        let mut s = self.cs.clone();
        match s.send(signal_code).await {
            Ok(_) => Ok({}),
            Err(e) => {
                error!("command executor: {}", e);
                return Err(errors::SignalSendError.into());
            }
        }
    }

    pub async fn call<T, R>(&self, request: T) -> connectors::Result<R>
    where
        T: ser::Serialize,
        T: traits::ObjectType,
        R: for<'de> de::Deserialize<'de>,
        R: traits::ObjectType,
    {
        let c = self.rt.get_command(T::get_type_name())?;
        let correlation_id = Uuid::new_v4().to_hyphenated().to_string();
        if c.path.contains_key(connectors::PROTO_HTTP) {
            let token = self
                .ac
                .get_client_basic_authorization_token(c.service_name.unwrap_or_default())?;
            let response = self
                .hcp
                .call(
                    &c.path.get(connectors::PROTO_HTTP).unwrap(),
                    T::get_type_name(),
                    &correlation_id,
                    token,
                    Body::from(serde_json::to_string(&request).unwrap()),
                )
                .await?;
            let reader = hyper::body::aggregate(response).await?.reader();
            let reply: Option<R> = serde_json::from_reader(reader).unwrap_or(None);
            if reply.is_some() {
                Ok(reply.unwrap())
            } else {
                Err(errors::BadReplyCommandError.into())
            }
        } else {
            let s = "{
            \"errorCode\": 0
        }";
            let r: R = serde_json::from_str(s).unwrap();
            Ok(r)
        }
    }
}

pub struct HttpCommandProducer {}

impl HttpCommandProducer {
    pub async fn new() -> connectors::Result<HttpCommandProducer> {
        Ok(HttpCommandProducer {})
    }

    pub async fn call(
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
            Err(errors::CallCommandError.into())
        }
    }
}

pub struct MqCommandProducer;

impl MqCommandProducer {
    pub async fn new() -> connectors::Result<MqCommandProducer> {
        Ok(MqCommandProducer {})
    }
}
