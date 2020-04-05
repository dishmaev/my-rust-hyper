use super::super::{connectors, entities::usr};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::postgres::{PgPool, PgQueryAs};
use std::sync::Arc;

pub struct UsrCollection {
    data_provider: Arc<connectors::SqlDbProvider>,
    exp_helper: &'static connectors::ExpHelper,
}

impl UsrCollection {
    pub fn new(
        data_provider: Arc<connectors::SqlDbProvider>,
        helper: &'static connectors::ExpHelper,
    ) -> UsrCollection {
        UsrCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(&self, ids: Option<Vec<i32>>) -> connectors::Result<Vec<usr::Usr>> {
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        if ids.is_none() {
            Ok(sqlx::query_as!(
                usr::Usr,
                r#"SELECT id,usr_name,usr_password FROM webapi.usr"#
            )
            .fetch_all(pool)
            .await?)
        } else {
            let query = self.exp_helper
            .get_select_int_exp("webapi.usr", "id", &ids.unwrap());
            let items: Vec<usr::Usr> = sqlx::query_as(
                &query
            )
            .fetch_all(pool).await?;
            Ok(items)
        }
    }
}
