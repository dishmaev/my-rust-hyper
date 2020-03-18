use super::connectors;
use serde::ser;

pub struct EventPublisher {
    http_event_producer: HttpEventProducer,
}

impl EventPublisher {
    pub async fn new() -> connectors::Result<EventPublisher> {
        Ok(EventPublisher {
            http_event_producer: HttpEventProducer {},
        })
    }

    pub fn send<T>(&self, items: Vec<T>)
    where
        T: ser::Serialize,
        T: std::fmt::Debug,
    {
        debug!("{:?}", items);
    }
}

pub struct HttpEventProducer;

impl HttpEventProducer {
    pub async fn new() -> connectors::Result<HttpEventProducer> {
        Ok(HttpEventProducer {})
    }
}
