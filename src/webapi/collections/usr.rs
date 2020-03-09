use super::super::{connectors, entities::usr};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::Row;
use std::sync::Arc;

pub struct UsrCollection {
    data_provider: Arc<connectors::SqlDbProvider>,
    exp_helper: &'static connectors::ExpHelper,
}

impl UsrCollection {
    pub fn new(data_provider: Arc<connectors::SqlDbProvider>, helper: &'static connectors::ExpHelper) -> UsrCollection {
        UsrCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(&self, ids: Option<Vec<i32>>) -> connectors::Result<Vec<usr::Usr>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        if ids.is_none() {
            Ok(sqlx::query_as!(
                usr::Usr,
                r#"SELECT id,usr_name,usr_password FROM webapi.usr"#
            )
            .fetch_all(&mut pool)
            .await?)
        } else {
            let recs = sqlx::query(
                &self
                    .exp_helper
                    .get_select_in_exp("webapi.usr", &ids.unwrap()),
            )
            .fetch_all(&mut pool)
            .await?;
            let mut items = Vec::<usr::Usr>::new();
            for rec in recs {
                items.push(usr::Usr {
                    id: rec.get(0),
                    usr_name: rec.get(1),
                    usr_password: rec.get(2),
                })
            }
            Ok(items)
        }
    }
}