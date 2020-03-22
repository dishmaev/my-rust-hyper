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
                if item.http_to.is_some() {
                    let token = self.ac.get_client_basic_authorization_token(
                        item.service_name.unwrap_or_default(),
                    )?;
                    self.hep
                        .send(
                            item.http_to.as_ref().unwrap(),
                            correlation_id,
                            token,
                            Body::from(serde_json::to_string(&items).unwrap()),
                        )
                        .await?;
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
        correlation_id: &str,
        bat: String,
        body: Body,
    ) -> connectors::Result<()> {
        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}?CorrelationId={}", to, correlation_id))
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
        Ok({})
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
