use super::{connectors, errors, traits};
use serde::ser;
use tokio::sync::mpsc;

pub struct EventPublisher {
    _http_event_producer: HttpEventProducer,
    sender: mpsc::Sender<String>,
}

impl EventPublisher {
    pub async fn new(sender: mpsc::Sender<String>) -> connectors::Result<EventPublisher> {
        Ok(EventPublisher {
            _http_event_producer: HttpEventProducer::new().await?,
            sender: sender,
        })
    }

    pub async fn send<T>(&self, correlation_id: &str, items: Vec<T>) -> connectors::Result<()>
    where
        T: ser::Serialize,
        T: traits::ObjectType,
    {
        debug!("correlation id {} object_type {} count {}", correlation_id, T::get_type_name(), items.len());
        let mut s = self.sender.clone();
        for item in items {
            match s.send(serde_json::to_string(&item).unwrap()).await {
                Ok(_) => {},
                Err(e) => {
                    error!("event publisher: {}", e);
                    return Err(errors::EventSendError.into())
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
