use super::{models, collections::*};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub struct ExpHelper {}

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
    pub error: error::ErrorCollection,
    pub usr: usr::UsrCollection,
    pub car: car::CarCollection,
    pub subscription: subscription::SubscriptionCollection,
}

impl DataConnector { 
    pub async fn new(_pg_db: &models::PgDb, _my_sql_db: &models::MySqlDb) -> Result<DataConnector> {
        let exp_helper: &'static ExpHelper = &ExpHelper::new();
        #[cfg(feature = "postgres")]
        let dp_arc = Arc::new(SqlDbProvider::new(&_pg_db.connection_string).await?);
        #[cfg(feature = "mysql")]
        let dp_arc = Arc::new(SqlDbProvider::new(&_my_sql_db.connection_string).await?);
        Ok(DataConnector {
            error: error::ErrorCollection::new(dp_arc.clone(), &exp_helper),
            usr: usr::UsrCollection::new(dp_arc.clone(), &exp_helper),
            car: car::CarCollection::new(dp_arc.clone(), &exp_helper),
            subscription: subscription::SubscriptionCollection::new(dp_arc.clone(), &exp_helper),
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
    pub async fn new(connection_string: &String) -> Result<SqlDbProvider> {
        debug!("connection string {}", connection_string);
        #[cfg(feature = "postgres")]
        let mut pool = PgPool::new(&connection_string).await.unwrap();
        #[cfg(feature = "mysql")]
        let mut pool = MySqlPool::new(&connection_string).await.unwrap();
        let error_items =
            sqlx::query_as!(error::Error, r#"SELECT id,error_name FROM webapi.error"#)
                .fetch_all(&mut pool)
                .await
                .unwrap_or(Vec::<error::Error>::new());
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

