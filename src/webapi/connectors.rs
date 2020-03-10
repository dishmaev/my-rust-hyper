#[cfg(not(test))]
use super::collections;
#[cfg(test)]
use super::tests::fakes;
use super::{entities, settings};
#[cfg(all(not(test), feature = "mysql"))]
use sqlx::MySqlPool;
#[cfg(all(not(test), feature = "postgres"))]
use sqlx::PgPool;
use std::collections::HashMap;
#[cfg(not(test))]
use std::sync::Arc;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[cfg(not(test))]
pub struct ExpHelper;

#[cfg(not(test))]
impl ExpHelper {
    fn new() -> &'static ExpHelper {
        &ExpHelper {}
    }

    pub fn get_ids_as_exp(&self, ids: &Vec<i32>) -> String {
        let mut result: String = String::with_capacity(100);
        for item in ids {
            if result.len() != 0 {
                result.push(',');
            }
            result.push_str(&item.to_string());
        }
        result
    }

    pub fn get_select_in_exp(&self, table: &str, ids: &Vec<i32>) -> String {
        format!(
            "SELECT * FROM {} WHERE id IN ({})",
            table,
            self.get_ids_as_exp(ids)
        )
    }

    pub fn get_delete_in_exp(&self, table: &str, ids: &Vec<i32>) -> String {
        format!(
            "DELETE FROM {} WHERE id IN ({})",
            table,
            self.get_ids_as_exp(ids)
        )
    }
}

pub struct DataConnector {
    pub error: HashMap<isize, String>,
    #[cfg(not(test))]
    pub usr: collections::usr::UsrCollection,
    #[cfg(test)]
    pub usr: fakes::usr::UsrCollection,
    #[cfg(not(test))]
    pub car: collections::car::CarCollection,
    #[cfg(test)]
    pub car: fakes::car::CarCollection,
    #[cfg(not(test))]
    pub subscription: collections::subscription::SubscriptionCollection,
    #[cfg(test)]
    pub subscription: fakes::subscription::SubscriptionCollection,
}

impl DataConnector {
    pub async fn new(
        _error: Option<HashMap<isize, String>>,
        _pg_db: Option<settings::PgDb>,
        _my_sql_db: Option<settings::MySqlDb>,
    ) -> Result<DataConnector> {
        #[cfg(not(test))]
        let _exp_helper: &'static ExpHelper = &ExpHelper::new();
        #[cfg(all(not(test), feature = "postgres"))]
        let dp = SqlDbProvider::new(&_pg_db.unwrap().connection_string).await?;
        #[cfg(all(not(test), feature = "mysql"))]
        let dp = SqlDbProvider::new(&_my_sql_db.unwrap().connection_string).await?;
        let mut error = HashMap::<isize, String>::new();
        if _error.is_some() {
            error.extend(_error.unwrap());
        }
        #[cfg(not(test))]
        error.extend(DataConnector::errors_as_hashmap(dp.get_errors().await?));
        #[cfg(not(test))]
        let _dp_arc = Arc::new(dp);
        Ok(DataConnector {
            error: error,
            #[cfg(not(test))]
            usr: collections::usr::UsrCollection::new(_dp_arc.clone(), &_exp_helper),
            #[cfg(test)]
            usr: fakes::usr::UsrCollection::new(),
            #[cfg(not(test))]
            car: collections::car::CarCollection::new(_dp_arc.clone(), &_exp_helper),
            #[cfg(test)]
            car: fakes::car::CarCollection::new(),
            #[cfg(not(test))]
            subscription: collections::subscription::SubscriptionCollection::new(
                _dp_arc.clone(),
                &_exp_helper,
            ),
            #[cfg(test)]
            subscription: fakes::subscription::SubscriptionCollection::new(),
        })
    }

    #[cfg(not(test))]
    fn errors_as_hashmap(items: Vec<entities::error::Error>) -> HashMap<isize, String> {
        let mut error = HashMap::<isize, String>::new();
        for item in items {
            error.insert(item.id as isize, item.error_name);
        }
        error
    }

    pub fn get_errors(&self, ids: Option<Vec<i32>>) -> Result<Vec<entities::error::Error>> {
        let is_ids = ids.is_some();
        let mut ids_as_ht = HashMap::<isize, _>::new();
        if is_ids {
            for item in &ids.unwrap() {
                ids_as_ht.insert(item.clone() as isize, 0);
            }
        }
        let mut items = Vec::<entities::error::Error>::new();
        for error in &self.error {
            if is_ids && !ids_as_ht.contains_key(error.0) {
                continue;
            }
            items.push(entities::error::Error {
                id: (error.0.clone() as i32),
                error_name: error.1.to_string(),
            });
        }
        Ok(items)
    }
}

#[cfg(not(test))]
pub struct SqlDbProvider {
    #[cfg(feature = "postgres")]
    pub pool: Arc<PgPool>,
    #[cfg(feature = "mysql")]
    pub pool: Arc<MySqlPool>,
}

#[cfg(not(test))]
impl SqlDbProvider {
    pub async fn new(connection_string: &String) -> Result<SqlDbProvider> {
        debug!("connection string {}", connection_string);
        #[cfg(feature = "postgres")]
        let pool = PgPool::new(&connection_string).await.unwrap();
        #[cfg(feature = "mysql")]
        let pool = MySqlPool::new(&connection_string).await.unwrap();
        Ok(SqlDbProvider {
            pool: Arc::new(pool),
        })
    }

    pub async fn get_errors(&self) -> Result<Vec<entities::error::Error>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.pool;
        Ok(sqlx::query_as!(
            entities::error::Error,
            r#"SELECT id,error_name FROM webapi.error"#
        )
        .fetch_all(&mut pool)
        .await
        .unwrap_or(Vec::<entities::error::Error>::new()))
    }
}
