use super::super::{connectors, entities::error};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::Row;
use std::sync::Arc;

pub struct ErrorCollection {
    _data_provider: Arc<connectors::SqlDbProvider>,
    _exp_helper: &'static connectors::ExpHelper,
}

impl ErrorCollection {
    pub fn new(data_provider: Arc<connectors::SqlDbProvider>, helper: &'static connectors::ExpHelper) -> ErrorCollection {
        ErrorCollection {
            _data_provider: data_provider,
            _exp_helper: &helper,
        }
    }

    pub async fn _get(&self, ids: Option<Vec<i32>>) -> connectors::Result<Vec<error::Error>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self._data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self._data_provider.pool;
        if ids.is_none() {
            Ok(
                sqlx::query_as!(error::Error, r#"SELECT id,error_name FROM webapi.error"#)
                    .fetch_all(&mut pool)
                    .await?,
            )
        } else {
            let recs = sqlx::query(
                &self
                    ._exp_helper
                    .get_select_in_exp("webapi.error", &ids.unwrap()),
            )
            .fetch_all(&mut pool)
            .await?;
            let mut items = Vec::<error::Error>::new();
            for rec in recs {
                items.push(error::Error {
                    id: rec.get(0),
                    error_name: rec.get(1),
                })
            }
            Ok(items)
        }
    }
}