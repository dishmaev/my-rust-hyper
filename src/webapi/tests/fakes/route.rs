use super::super::super::{connectors, entities::route, errors};

pub struct RouteCollection {
    items: Vec<route::Route>,
}

impl RouteCollection {
    pub fn new() -> RouteCollection {
        let items = vec![];
        RouteCollection { items: items }
    }

    pub async fn get(
        &self,
        _services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::Route>> {
        Ok(self.items.clone())
    }

    pub async fn get_command(
        &self,
        _services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::ServiceCommand>> {
        Ok(vec![])
    }

    pub async fn get_event(
        &self,
        _services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::ServiceEvent>> {
        Ok(vec![])
    }

    pub async fn get_subscription(
        &self,
        _services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::ServiceSubscription>> {
        Ok(vec![])
    }

    pub async fn get_service(
        &self,
        _services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::Service>> {
        Ok(vec![])
    }

    pub async fn get_service_path(
        &self,
        _services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::ServicePath>> {
        Ok(vec![])
    }

    pub async fn add(
        &self,
        _items: Vec<route::Route>,
    ) -> connectors::Result<(errors::ErrorCode, Option<Vec<String>>)> {
        Ok((errors::ErrorCode::ReplyOk, None))
    }

    pub async fn remove(&self, _ids: Vec<String>) -> connectors::Result<errors::ErrorCode> {
        Ok(errors::ErrorCode::ReplyOk)
    }
}
