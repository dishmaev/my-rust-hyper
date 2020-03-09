use super::super::{connectors, entities::subscription, errors};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::Row;
use std::sync::Arc;

pub struct SubscriptionCollection {
    exp_helper: &'static connectors::ExpHelper,
    data_provider: Arc<connectors::SqlDbProvider>,
}

impl SubscriptionCollection {
    pub fn new(
        data_provider: Arc<connectors::SqlDbProvider>,
        helper: &'static connectors::ExpHelper,
    ) -> SubscriptionCollection {
        SubscriptionCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(
        &self,
        ids: Option<Vec<i32>>,
    ) -> connectors::Result<Vec<subscription::Subscription>> {
        let mut items = Vec::<subscription::Subscription>::new();
        items.push(subscription::Subscription {
            id: Some(1),
            object_name: Some("car".to_string()),
            event_name: Some("ondelete".to_string()),
            call_back: "http://my.ru".to_string(),
        });
        Ok(items)
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
