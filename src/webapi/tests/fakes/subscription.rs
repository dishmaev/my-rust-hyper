use super::super::super::{connectors, entities::subscription, errors};

pub struct SubscriptionCollection {
    items: Vec<subscription::Subscription>,
}

impl SubscriptionCollection {
    pub fn new() -> SubscriptionCollection {
        let items = vec![];
        SubscriptionCollection { items: items }
    }
    pub async fn get(
        &self,
        ids: Option<Vec<i32>>,
    ) -> connectors::Result<Vec<subscription::Subscription>> {
        Ok(self.items.clone())
    }

    pub async fn subscribe(
        &self,
        object_name: &str,
        event_name: &str,
        call_back: &str,
    ) -> connectors::Result<errors::ErrorCode> {
        Ok(errors::ErrorCode::ReplyOk)
    }

    pub async fn unsubscribe(
        &self,
        object_name: &str,
        event_name: &str,
        call_back: &str,
    ) -> connectors::Result<errors::ErrorCode> {
        Ok(errors::ErrorCode::ReplyOk)
    }
}
