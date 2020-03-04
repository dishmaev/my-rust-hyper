use super::{errors, models};
use serde::Serialize;
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::Row;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddReply {
    pub error_code: errors::ErrorCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i32>>,
}

pub struct ExpHelper {}

impl ExpHelper {
    fn new() -> &'static ExpHelper {
        &ExpHelper {}
    }

    fn get_ids_as_exp(&self, ids: &Vec<i32>) -> String {
        let mut result: String = String::with_capacity(100);
        for item in ids {
            if result.len() != 0 {
                result.push(',');
            }
            result.push_str(&item.to_string());
        }
        result
    }

    fn get_select_in_exp(&self, table: &str, ids: &Vec<i32>) -> String {
        format!(
            "SELECT * FROM {} WHERE id IN ({})",
            table,
            self.get_ids_as_exp(ids)
        )
    }
    fn get_delete_in_exp(&self, table: &str, ids: &Vec<i32>) -> String {
        format!(
            "DELETE FROM {} WHERE id IN ({})",
            table,
            self.get_ids_as_exp(ids)
        )
    }
}

pub struct DataConnector {
    pub error: ErrorCollection,
    pub usr: UsrCollection,
    pub car: CarCollection,
    pub subscription: SubscriptionCollection,
}

impl DataConnector {
    pub async fn new(_pg_db: models::PgDb, _my_sql_db: models::MySqlDb) -> Result<DataConnector> {
        let exp_helper: &'static ExpHelper = &ExpHelper::new();
        #[cfg(feature = "postgres")]
        let dp_arc = Arc::new(SqlDbProvider::new(_pg_db.connection_string).await?);
        #[cfg(feature = "mysql")]
        let dp_arc = Arc::new(SqlDbProvider::new(_my_sql_db.connection_string).await?);
        Ok(DataConnector {
            error: ErrorCollection::new(dp_arc.clone(), &exp_helper),
            usr: UsrCollection::new(dp_arc.clone(), &exp_helper),
            car: CarCollection::new(dp_arc.clone(), &exp_helper),
            subscription: SubscriptionCollection::new(dp_arc.clone(), &exp_helper),
        })
    }
}

pub struct SqlDbProvider {
    #[cfg(feature = "postgres")]
    pub pool: Arc<PgPool>,
    #[cfg(feature = "mysql")]
    pub pool: Arc<MySqlPool>,
    pub error: HashMap<isize, String>,
}

impl SqlDbProvider {
    pub async fn new(connection_string: String) -> Result<SqlDbProvider> {
        #[cfg(feature = "postgres")]
        let mut pool = PgPool::new(&connection_string).await.unwrap();
        #[cfg(feature = "mysql")]
        let mut pool = MySqlPool::new(&connection_string).await.unwrap();
        let error_items =
            sqlx::query_as!(models::Error, r#"SELECT id,error_name FROM webapi.error"#)
                .fetch_all(&mut pool)
                .await
                .unwrap_or(Vec::<models::Error>::new());
        let mut error = HashMap::<isize, String>::new();
        for item in error_items {
            error.insert(item.id as isize, item.error_name);
        }
        Ok(SqlDbProvider {
            pool: Arc::new(pool),
            error: error,
        })
    }
}

pub struct UsrCollection {
    data_provider: Arc<SqlDbProvider>,
    exp_helper: &'static ExpHelper,
}

impl UsrCollection {
    pub fn new(data_provider: Arc<SqlDbProvider>, helper: &'static ExpHelper) -> UsrCollection {
        UsrCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(&self, ids: Option<Vec<i32>>) -> Result<Vec<models::Usr>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        if ids.is_none() {
            Ok(sqlx::query_as!(
                models::Usr,
                r#"SELECT id,usr_name,usr_password FROM webapi.usr"#
            )
            .fetch_all(&mut pool)
            .await?)
        } else {
            let items = sqlx::query(
                &self
                    .exp_helper
                    .get_select_in_exp("webapi.usr", &ids.unwrap()),
            )
            .fetch_all(&mut pool)
            .await?;
            let mut result = Vec::<models::Usr>::new();
            for item in items {
                result.push(models::Usr {
                    id: item.get(0),
                    usr_name: item.get(1),
                    usr_password: item.get(2),
                })
            }
            Ok(result)
        }
    }
}

pub struct ErrorCollection {
    _data_provider: Arc<SqlDbProvider>,
    _exp_helper: &'static ExpHelper,
}

impl ErrorCollection {
    pub fn new(data_provider: Arc<SqlDbProvider>, helper: &'static ExpHelper) -> ErrorCollection {
        ErrorCollection {
            _data_provider: data_provider,
            _exp_helper: &helper,
        }
    }

    pub async fn _get(&self, ids: Option<Vec<i32>>) -> Result<Vec<models::Error>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self._data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self._data_provider.pool;
        if ids.is_none() {
            Ok(
                sqlx::query_as!(models::Error, r#"SELECT id,error_name FROM webapi.error"#)
                    .fetch_all(&mut pool)
                    .await?,
            )
        } else {
            let items = sqlx::query(
                &self
                    ._exp_helper
                    .get_select_in_exp("webapi.error", &ids.unwrap()),
            )
            .fetch_all(&mut pool)
            .await?;
            let mut result = Vec::<models::Error>::new();
            for item in items {
                result.push(models::Error {
                    id: item.get(0),
                    error_name: item.get(1),
                })
            }
            Ok(result)
        }
    }
}

pub struct CarCollection {
    data_provider: Arc<SqlDbProvider>,
    exp_helper: &'static ExpHelper,
}

impl CarCollection {
    pub fn new(data_provider: Arc<SqlDbProvider>, helper: &'static ExpHelper) -> CarCollection {
        CarCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(&self, ids: Option<Vec<i32>>) -> Result<Vec<models::Car>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        if ids.is_none() {
            Ok(
                sqlx::query_as!(models::Car, r#"SELECT id,car_name FROM webapi.car"#)
                    .fetch_all(&mut pool)
                    .await?,
            )
        } else {
            let items = sqlx::query(
                &self
                    .exp_helper
                    .get_select_in_exp("webapi.car", &ids.unwrap()),
            )
            .fetch_all(&mut pool)
            .await?;
            let mut result = Vec::<models::Car>::new();
            for item in items {
                result.push(models::Car {
                    id: item.get(0),
                    car_name: item.get(1),
                })
            }
            Ok(result)
        }
    }

    pub async fn add(&self, items: Vec<models::Car>) -> Result<AddReply> {
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

    pub async fn update(&self, items: Vec<models::Car>) -> Result<Reply> {
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
    pub async fn delete(&self, ids: Vec<i32>) -> Result<Reply> {
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

pub struct SubscriptionCollection {
    exp_helper: &'static ExpHelper,
    data_provider: Arc<SqlDbProvider>,
}

impl SubscriptionCollection {
    pub fn new(
        data_provider: Arc<SqlDbProvider>,
        helper: &'static ExpHelper,
    ) -> SubscriptionCollection {
        SubscriptionCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(&self, ids: Option<Vec<i32>>) -> Result<Vec<models::Subscription>> {
        let mut items = Vec::<models::Subscription>::new();
        items.push(models::Subscription {
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
    ) -> Result<Reply> {
        Ok(Reply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
        })
    }

    pub async fn unsubscribe(
        &self,
        object_name: &str,
        event_name: &str,
        call_back: &str,
    ) -> Result<Reply> {
        Ok(Reply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
        })
    }
}
