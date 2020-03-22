use super::{connectors, errors, router, traits, workers};
use serde::ser;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct EventPublisher {
    rt: Arc<router::Router>,
    _control_sender: mpsc::Sender<workers::SignalCode>,
    _http_event_producer: HttpEventProducer,
    _mq_event_producer: MqEventProducer,
}

impl EventPublisher {
    pub async fn new(
        rt: Arc<router::Router>,
        control_sender: mpsc::Sender<workers::SignalCode>,
    ) -> connectors::Result<EventPublisher> {
        Ok(EventPublisher {
            rt: rt,
            _control_sender: control_sender,
            _http_event_producer: HttpEventProducer::new().await?,
            _mq_event_producer: MqEventProducer::new().await?,
        })
    }

    pub async fn send_signal(&self, signal_code: workers::SignalCode) -> connectors::Result<()> {
        let mut s = self._control_sender.clone();
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
            "correlation id {} object_type {} count {}",
            correlation_id,
            T::get_type_name(),
            items.len()
        );
        if let Some(s) = self.rt.get_subscriptions(T::get_type_name()){
            for item in s {
                if item.http_to.is_some() {
                    
                } else {
                }
            }
        }
        Ok({})
    }
}

pub struct HttpEventProducer;

impl HttpEventProducer {
    pub async fn new() -> connectors::Result<HttpEventProducer> {
        Ok(HttpEventProducer {})
    }
}

pub struct MqEventProducer;

impl MqEventProducer {
    pub async fn new() -> connectors::Result<MqEventProducer> {
        Ok(MqEventProducer {})
    }
}
