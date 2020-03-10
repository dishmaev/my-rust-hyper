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
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        if ids.is_none() {
            Ok(sqlx::query_as!(
                subscription::Subscription,
                r#"SELECT id, object_name, event_name, call_back FROM webapi.subscription"#
            )
            .fetch_all(&mut pool)
            .await?)
        } else {
            let recs = sqlx::query(
                &self
                    .exp_helper
                    .get_select_in_exp("webapi.subscription", &ids.unwrap()),
            )
            .fetch_all(&mut pool)
            .await?;
            let mut items = Vec::<subscription::Subscription>::new();
            for rec in recs {
                items.push(subscription::Subscription {
                    id: rec.get(0),
                    object_name: rec.get(1),
                    event_name: rec.get(2),
                    call_back: rec.get(3),
                })
            }
            Ok(items)
        }
    }

    pub async fn subscribe(
        &self,
        _object_name: &str,
        _event_name: &str,
        _call_back: &str,
    ) -> connectors::Result<errors::ErrorCode> {
        Ok(errors::ErrorCode::ReplyOk)
    }

    pub async fn unsubscribe(
        &self,
        _object_name: &str,
        _event_name: &str,
        _call_back: &str,
    ) -> connectors::Result<errors::ErrorCode> {
        Ok(errors::ErrorCode::ReplyOk)
    }
}
