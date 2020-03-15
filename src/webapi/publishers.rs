use super::connectors;

pub struct EventPublisher{
    http_event_producer: HttpEventProducer
}

impl EventPublisher{
    pub async fn new() -> connectors::Result<EventPublisher> {
        Ok(EventPublisher{ http_event_producer: HttpEventProducer{}})
    }
}

pub struct HttpEventProducer;

impl HttpEventProducer {
    pub async fn new() -> connectors::Result<HttpEventProducer> {
        Ok(HttpEventProducer{})
    }
}