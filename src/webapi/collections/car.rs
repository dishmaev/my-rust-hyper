use super::super::{connectors, entities::car, errors, providers};
#[cfg(feature = "postgres")]
use sqlx::postgres::PgPool;
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
use sqlx::Done;
use std::convert::TryFrom;
use std::sync::Arc;

pub struct CarCollection {
    data_provider: Arc<providers::SqlDbProvider>,
    exp_helper: &'static connectors::ExpHelper,
}

impl CarCollection {
    pub fn new(
        data_provider: Arc<providers::SqlDbProvider>,
        helper: &'static connectors::ExpHelper,
    ) -> CarCollection {
        CarCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(&self, ids: Option<Vec<i32>>) -> connectors::Result<Vec<car::Car>> {
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        if ids.is_none() {
            Ok(
                sqlx::query_as!(car::Car, r#"SELECT id as "id?",car_name FROM webapi.car"#)
                    .fetch_all(pool)
                    .await?,
            )
        } else {
            let query = self
                .exp_helper
                .get_select_int_exp("webapi.car", "id", &ids.unwrap());
            let items: Vec<car::Car> = sqlx::query_as(&query).fetch_all(pool).await?;
            Ok(items)
        }
    }

    pub async fn add(
        &self,
        items: Vec<car::Car>,
    ) -> connectors::Result<(errors::ErrorCode, Option<Vec<i32>>)> {
        let mut ids = Vec::<i32>::new();
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut tx = pool.begin().await?;
        for item in items {
            #[cfg(feature = "postgres")]
            match sqlx::query!(
                r#"INSERT INTO webapi.car ( car_name ) VALUES ( $1 ) RETURNING id"#,
                item.car_name
            )
            .fetch_one(&mut tx)
            .await
            {
                Ok(rec) => ids.push(rec.id),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_cars db insert: {}", e);
                    return Ok((errors::ErrorCode::DatabaseError, None));
                }
            };
            #[cfg(feature = "mysql")]
            match sqlx::query(r#"INSERT INTO webapi.car ( car_name ) VALUES ( ? )"#)
                .bind(item.car_name)
                .execute(&mut tx)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_cars db insert: {}", e);
                    return Ok((errors::ErrorCode::DatabaseError, None));
                }
            };
            #[cfg(feature = "mysql")]
            match sqlx::query(r#"SELECT LAST_INSERT_ID() AS id;"#)
                .fetch_one(&mut tx)
                .await
            {
                Ok(rec) => ids.push(rec.get(0)),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_cars db insert: {}", e);
                    return Ok((errors::ErrorCode::DatabaseError, None));
                }
            };
        }
        match tx.commit().await {
            Ok(_) => {}
            Err(e) => {
                error!("add_cars db commit: {}", e);
                return Ok((errors::ErrorCode::DatabaseError, None));
            }
        }
        Ok((errors::ErrorCode::ReplyOk, Some(ids)))
    }

    pub async fn change(&self, items: Vec<car::Car>) -> connectors::Result<errors::ErrorCode> {
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut tx = pool.begin().await?;
        let mut count: u64 = 0;
        for item in &items {
            #[cfg(feature = "postgres")]
            match sqlx::query!(
                r#"UPDATE webapi.car SET car_name = $1 WHERE id = $2"#,
                item.car_name,
                item.id.unwrap_or(0)
            )
            .execute(&mut tx)
            .await
            {
                Ok(ret) => count += ret.rows_affected(),
                Err(e) => {
                    error!("update_cars db update: {}", e);
                    tx.rollback().await?;
                    return Ok(errors::ErrorCode::DatabaseError);
                }
            };
            #[cfg(feature = "mysql")]
            match sqlx::query!(
                r#"UPDATE car SET car_name = ? WHERE id = ?"#,
                item.car_name,
                item.id.unwrap_or(0)
            )
            .execute(&mut tx)
            .await
            {
                Ok(ret) => count += ret,
                Err(e) => {
                    error!("update_cars db update: {}", e);
                    tx.rollback().await?;
                    return Ok(errors::ErrorCode::DatabaseError);
                }
            };
        }
        if items.len() == usize::try_from(count).unwrap() {
            match tx.commit().await {
                Ok(_) => {}
                Err(e) => {
                    error!("update_cars db commit: {}", e);
                    return Ok(errors::ErrorCode::DatabaseError);
                }
            }
            Ok(errors::ErrorCode::ReplyOk)
        } else {
            tx.rollback().await?;
            Ok(errors::ErrorCode::NotFoundError)
        }
    }

    pub async fn remove(&self, ids: Vec<i32>) -> connectors::Result<errors::ErrorCode> {
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut tx = pool.begin().await?;
        match sqlx::query(&self.exp_helper.get_delete_int_exp("webapi.car", "id", &ids))
            .execute(&mut tx)
            .await
        {
            Ok(ret) => {
                if ids.len() == usize::try_from(ret.rows_affected()).unwrap() {
                    match tx.commit().await {
                        Ok(_) => Ok(errors::ErrorCode::ReplyOk),
                        Err(e) => {
                            error!("remove_cars db commit: {}", e);
                            return Ok(errors::ErrorCode::DatabaseError);
                        }
                    }
                } else {
                    tx.rollback().await?;
                    Ok(errors::ErrorCode::NotFoundError)
                }
            }
            Err(e) => {
                error!("remove_cars db delete: {}", e);
                tx.rollback().await?;
                Ok(errors::ErrorCode::DatabaseError)
            }
        }
    }
}
