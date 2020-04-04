use super::{access, connectors, errors, router, traits, workers};
use hyper::{Body, Client, Method, Request, StatusCode};
use serde::ser;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct EventPublisher {
    ac: Arc<access::AccessChecker>,
    rt: Arc<router::Router>,
    cs: mpsc::Sender<workers::SignalCode>,
    hep: HttpEventProducer,
    mep: MqEventProducer,
}

impl EventPublisher {
    pub async fn new(
        ac: Arc<access::AccessChecker>,
        rt: Arc<router::Router>,
        cs: mpsc::Sender<workers::SignalCode>,
    ) -> connectors::Result<EventPublisher> {
        Ok(EventPublisher {
            ac: ac,
            rt: rt,
            cs: cs,
            hep: HttpEventProducer::new().await?,
            mep: MqEventProducer::new().await?,
        })
    }

    pub async fn send_signal(&self, signal_code: workers::SignalCode) -> connectors::Result<()> {
        let mut s = self.cs.clone();
        match s.send(signal_code).await {
            Ok(_) => Ok({}),
            Err(e) => {
                error!("event publisher: {}", e);
                return Err(errors::SignalSendError.into());
            }
        }
    }

    pub async fn send<T>(&self, correlation_id: &str, items: Vec<T>) -> connectors::Result<()>
    where
        T: ser::Serialize,
        T: traits::ObjectType,
    {
        debug!(
            "correlation id {} publish {} count event {}",
            correlation_id,
            items.len(),
            T::get_type_name()
        );
        if let Some(s) = self.rt.get_subscriptions(T::get_type_name()) {
            for item in s {
                if item.path.contains_key(connectors::PROTO_HTTP) {
                    let token = self.ac.get_client_basic_authorization_token(
                        item.service_name.unwrap_or_default(),
                    )?;
                    match self.hep
                        .send(
                            item.path.get(connectors::PROTO_HTTP).unwrap(),
                            T::get_type_name(),
                            correlation_id,
                            token,
                            Body::from(serde_json::to_string(&items).unwrap()),
                        )
                        .await{
                            Ok(_) => {},
                            Err(e) => {
                                warn!("correlation id {} object type {} send error {}", correlation_id, T::get_type_name(), e);
                            }
                        }
                } else {
                }
            }
        }
        Ok({})
    }
}

pub struct HttpEventProducer {}

impl HttpEventProducer {
    pub async fn new() -> connectors::Result<HttpEventProducer> {
        Ok(HttpEventProducer {})
    }

    pub async fn send(
        &self,
        to: &str,
        object_type: &str,
        correlation_id: &str,
        bat: String,
        body: Body,
    ) -> connectors::Result<()> {
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
        if resp.status() == StatusCode::OK {
            Ok({})
        }
        else{
            Err(errors::SendEventError.into())
        }
    }
}

pub struct MqEventProducer;

impl MqEventProducer {
    pub async fn new() -> connectors::Result<MqEventProducer> {
        Ok(MqEventProducer {})
    }

    pub async fn send() -> connectors::Result<()> {
        Ok({})
    }
}
