use super::super::{connectors, entities::executor, errors};
#[cfg(feature = "postgres")]
use sqlx::postgres::{PgPool, PgQueryAs};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
use sqlx::{Cursor, Executor, Row};
use std::sync::Arc;

pub struct SendedAsyncCommandCollection {
    data_provider: Arc<connectors::SqlDbProvider>,
    exp_helper: &'static connectors::ExpHelper,
}

impl SendedAsyncCommandCollection {
    pub fn new(
        data_provider: Arc<connectors::SqlDbProvider>,
        helper: &'static connectors::ExpHelper,
    ) -> SendedAsyncCommandCollection {
        SendedAsyncCommandCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(
        &self,
        ids: Option<Vec<String>>,
    ) -> connectors::Result<Vec<executor::SendedAsyncCommand>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;

        if ids.is_none() {
            Ok(sqlx::query_as!(
                executor::SendedAsyncCommand,
                r#"SELECT id,object_type,proto,"state_to",created_at 
                    FROM webapi.v_sended_async_command"#
            )
            .fetch_all(&mut pool)
            .await?)
        } else {
            let query = self.exp_helper
            .get_select_str_exp("webapi.v_sended_async_command", "id", &ids.unwrap());
            let items: Vec<executor::SendedAsyncCommand> = sqlx::query_as(
                &query
            )
            .fetch_all(&mut pool).await?;

            // let query = self.exp_helper
            // .get_select_str_exp("webapi.v_sended_async_command", "id", &ids.unwrap());
            // let mut cursor = sqlx::query(
            //     &query
            // )
            // .fetch(&mut pool);
            // let mut items = Vec::<executor::SendedAsyncCommand>::new();
            // while let Some(rec) = cursor.next().await? {
            //     items.push(executor::SendedAsyncCommand {
            //         id: rec.get(0),
            //         object_type: rec.get(1),
            //         proto: rec.get(2),
            //         state_to: rec.get(3),
            //         created_at: rec.get(4),
            //     })
            // }            
            Ok(items)
        }
    }

    // pub async fn add(
    //     &self,
    //     items: Vec<executor::SendedAsyncCommand>,
    // ) -> connectors::Result<(errors::ErrorCode, Option<Vec<String>>)> {
    // }

    // pub async fn remove(
    //     &self,
    //     ids: Vec<String>,
    // ) -> connectors::Result<errors::ErrorCode> {
    // }
}

pub struct ReceivedAsyncCommandCollection {
    data_provider: Arc<connectors::SqlDbProvider>,
    exp_helper: &'static connectors::ExpHelper,
}

impl ReceivedAsyncCommandCollection {
    pub fn new(
        data_provider: Arc<connectors::SqlDbProvider>,
        helper: &'static connectors::ExpHelper,
    ) -> ReceivedAsyncCommandCollection {
        ReceivedAsyncCommandCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    // pub async fn get(
    //     &self,
    //     ids: Option<Vec<String>>,
    // ) -> connectors::Result<Vec<executor::ReceivedAsyncCommand>> {
    // }

    // pub async fn add(
    //     &self,
    //     items: Vec<executor::ReceivedAsyncCommand>,
    // ) -> connectors::Result<(errors::ErrorCode, Option<Vec<String>>)> {
    // }

    // pub async fn update_state(
    //     &self,
    //     state: String,
    //     ids: Vec<String>,
    // ) -> connectors::Result<errors::ErrorCode> {
    // }

    // pub async fn remove(
    //     &self,
    //     ids: Vec<String>,
    // ) -> connectors::Result<errors::ErrorCode> {
    // }
}
