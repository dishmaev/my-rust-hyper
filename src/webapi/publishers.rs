use super::{access, connectors, errors, providers, router, traits, workers};
use hyper::Body;
use serde::ser;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct EventPublisher {
    ac: Arc<access::AccessChecker>,
    rt: Arc<router::Router>,
    hp: providers::HttpProvider,
    mp: providers::MqProvider,
    _cs: mpsc::Sender<workers::SignalCode>,
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
            hp: providers::HttpProvider::new().await?,
            mp: providers::MqProvider::new().await?,
            _cs: cs,
        })
    }

    pub async fn send_signal(&self, signal_code: workers::SignalCode) -> connectors::Result<()> {
        let s = self._cs.clone();
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
                let mut prop = HashMap::<&str, &str>::new();
                prop.insert("correlation_id", correlation_id);
                prop.insert("object_type", T::get_type_name());
                if item.path.contains_key(&providers::Proto::Http.to_string()) {
                    let token = self.ac.get_client_basic_authorization_token(
                        item.service_name.as_ref().unwrap(),
                    )?;
                    match self
                        .hp
                        .execute(
                            item.path.get(&providers::Proto::Http.to_string()).unwrap(),
                            prop,
                            token,
                            Body::from(serde_json::to_string(&items).unwrap()),
                        )
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            warn!(
                                "correlation id {} object type {} send error {}",
                                correlation_id,
                                T::get_type_name(),
                                e
                            );
                        }
                    }
                } else if item.path.contains_key(&providers::Proto::Mq.to_string()) {
                    match self
                        .mp
                        .execute(
                            item.path.get(&providers::Proto::Http.to_string()).unwrap(),
                            prop,
                            Body::from(serde_json::to_string(&items).unwrap()),
                        )
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            warn!(
                                "correlation id {} object type {} send error {}",
                                correlation_id,
                                T::get_type_name(),
                                e
                            );
                        }
                    }
                } else {
                    warn!(
                        "correlation id {} object type {} supported proto not found error",
                        correlation_id,
                        T::get_type_name()
                    );
                }
            }
        }
        Ok({})
    }
}
