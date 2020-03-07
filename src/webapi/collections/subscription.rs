use super::super::{errors, models, connectors};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::Row;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,

    pub call_back: String,
}

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

    pub async fn get(&self, ids: Option<Vec<i32>>) -> connectors::Result<Vec<Subscription>> {
        let mut items = Vec::<Subscription>::new();
        items.push(Subscription {
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
    ) -> connectors::Result<models::Reply> {
        Ok(models::Reply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
        })
    }

    pub async fn unsubscribe(
        &self,
        object_name: &str,
        event_name: &str,
        call_back: &str,
    ) -> connectors::Result<models::Reply> {
        Ok(models::Reply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
        })
    }
}
