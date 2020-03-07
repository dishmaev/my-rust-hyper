use super::super::{errors, models, connectors};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::Row;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Usr {
    pub id: i32,
    pub usr_name: String,
    pub usr_password: String,
}

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

    pub async fn get(&self, ids: Option<Vec<i32>>) -> connectors::Result<Vec<Usr>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        if ids.is_none() {
            Ok(sqlx::query_as!(
                Usr,
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
            let mut items = Vec::<Usr>::new();
            for rec in recs {
                items.push(Usr {
                    id: rec.get(0),
                    usr_name: rec.get(1),
                    usr_password: rec.get(2),
                })
            }
            Ok(items)
        }
    }
}