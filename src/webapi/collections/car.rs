use super::super::{errors, models, connectors};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::Row;
use std::sync::Arc;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Car {
    pub id: Option<i32>,
    pub car_name: String,
}

pub struct CarCollection {
    data_provider: Arc<connectors::SqlDbProvider>,
    exp_helper: &'static connectors::ExpHelper,
}

impl CarCollection {
    pub fn new(data_provider: Arc<connectors::SqlDbProvider>, helper: &'static connectors::ExpHelper) -> CarCollection {
        CarCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(&self, ids: Option<Vec<i32>>) -> connectors::Result<Vec<Car>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        if ids.is_none() {
            Ok(
                sqlx::query_as!(Car, r#"SELECT id,car_name FROM webapi.car"#)
                    .fetch_all(&mut pool)
                    .await?,
            )
        } else {
            let recs = sqlx::query(
                &self
                    .exp_helper
                    .get_select_in_exp("webapi.car", &ids.unwrap()),
            )
            .fetch_all(&mut pool)
            .await?;
            let mut items = Vec::<Car>::new();
            for rec in recs {
                items.push(Car {
                    id: rec.get(0),
                    car_name: rec.get(1),
                })
            }
            Ok(items)
        }
    }

    pub async fn add(&self, items: Vec<Car>) -> connectors::Result<models::AddReply> {
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
                    println!("add_cars db insert error: {}", e);
                    return Ok(get_error_add_reply!(
                        errors::ErrorCode::ReplyErrorDatabase,
                        self.data_provider.error
                    ));
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
                    println!("add_cars db insert error: {}", e);
                    return Ok(get_error_add_reply!(
                        errors::ErrorCode::ReplyErrorDatabase,
                        self.data_provider.error
                    ));
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
                    println!("add_cars db insert error: {}", e);
                    return Ok(get_error_add_reply!(
                        errors::ErrorCode::ReplyErrorDatabase,
                        self.data_provider.error
                    ));
                }
            };
        }
        match tx.commit().await {
            Ok(_) => {}
            Err(e) => {
                println!("add_cars db commit error: {}", e);
                return Ok(get_error_add_reply!(
                    errors::ErrorCode::ReplyErrorDatabase,
                    self.data_provider.error
                ));
            }
        }
        Ok(get_ok_add_reply!(ids))
    }

    pub async fn update(&self, items: Vec<Car>) -> connectors::Result<models::Reply> {
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
                Ok(ret) => count += ret,
                Err(e) => {
                    println!("update_cars db update error: {}", e);
                    tx.rollback().await?;
                    return Ok(get_error_reply!(
                        errors::ErrorCode::ReplyErrorDatabase,
                        self.data_provider.error
                    ));
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
                    println!("update_cars db update error: {}", e);
                    tx.rollback().await?;
                    return Ok(get_error_reply!(
                        errors::ErrorCode::ReplyErrorDatabase,
                        self.data_provider.error
                    ));
                }
            };
        }
        if items.len() == usize::try_from(count).unwrap() {
            match tx.commit().await {
                Ok(_) => {}
                Err(e) => {
                    println!("update_cars db commit error: {}", e);
                    return Ok(get_error_reply!(
                        errors::ErrorCode::ReplyErrorDatabase,
                        self.data_provider.error
                    ));
                }
            }
            Ok(get_ok_reply!())
        } else {
            tx.rollback().await?;
            Ok(get_error_reply!(
                errors::ErrorCode::ReplyErrorNotFound,
                self.data_provider.error
            ))
        }
    }
    
    pub async fn delete(&self, ids: Vec<i32>) -> connectors::Result<models::Reply> {
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut tx = pool.begin().await?;
        match sqlx::query(&self.exp_helper.get_delete_in_exp("webapi.car", &ids))
            .execute(&mut tx)
            .await
        {
            Ok(ret) => {
                if ids.len() == usize::try_from(ret).unwrap() {
                    match tx.commit().await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("delete_cars db commit error: {}", e);
                            return Ok(get_error_reply!(
                                errors::ErrorCode::ReplyErrorDatabase,
                                self.data_provider.error
                            ));
                        }
                    }
                    Ok(get_ok_reply!())
                } else {
                    tx.rollback().await?;
                    Ok(get_error_reply!(
                        errors::ErrorCode::ReplyErrorNotFound,
                        self.data_provider.error
                    ))
                }
            }
            Err(e) => {
                println!("delete_cars db delete error: {}", e);
                tx.rollback().await?;
                Ok(get_error_reply!(
                    errors::ErrorCode::ReplyErrorDatabase,
                    self.data_provider.error
                ))
            }
        }
    }
}
