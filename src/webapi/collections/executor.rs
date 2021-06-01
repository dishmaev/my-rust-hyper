use super::super::{connectors, entities::executor, errors, providers};
use futures::TryStreamExt;
#[cfg(feature = "postgres")]
use sqlx::postgres::PgPool;
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
use sqlx::Row;
use std::convert::TryFrom;
use std::sync::Arc;

pub struct SendedAsyncCommandCollection {
    data_provider: Arc<providers::SqlDbProvider>,
    exp_helper: &'static connectors::ExpHelper,
}

impl SendedAsyncCommandCollection {
    pub fn new(
        data_provider: Arc<providers::SqlDbProvider>,
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
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut items = Vec::<executor::SendedAsyncCommand>::new();
        if ids.is_none() {
            let recs = sqlx::query!(
                r#"SELECT id, object_type, "service_name", "state", change_state_event, added_at, state_changed_at 
                    FROM webapi.v_sended_async_command"#
            )
            .fetch_all(pool)
            .await?;
            for rec in recs {
                items.push(executor::SendedAsyncCommand {
                    id: rec.id.unwrap(),
                    object_type: rec.object_type.unwrap(),
                    service_name: rec.service_name.unwrap(),
                    state: rec.state.unwrap(),
                    change_state_event: rec.change_state_event.unwrap(),
                    added_at: rec.added_at.unwrap(),
                    state_changed_at: rec.state_changed_at.unwrap(),
                    history: None,
                })
            }
        } else {
            let query = self.exp_helper.get_select_str_exp(
                "webapi.v_sended_async_command",
                "id",
                &ids.unwrap(),
            );
            let mut cursor = sqlx::query(&query).fetch(pool);
            while let Some(rec) = cursor.try_next().await? {
                items.push(executor::SendedAsyncCommand {
                    id: rec.get(0),
                    object_type: rec.get(1),
                    service_name: rec.get(2),
                    state: rec.get(3),
                    change_state_event: rec.get(4),
                    added_at: rec.get(5),
                    state_changed_at: rec.get(6),
                    history: None,
                })
            }
        }
        for mut item in &mut items {
            let recs = sqlx::query!(
                r#"SELECT command_id, "state", added_at
                    FROM webapi.v_sended_async_command_state_history
                        WHERE command_id = $1"#,
                &item.id
            )
            .fetch_all(pool)
            .await?;
            if recs.len() > 0 {
                let mut p = Vec::<executor::SendedAsyncCommandHistory>::new();
                for rec in recs {
                    p.push(executor::SendedAsyncCommandHistory {
                        command_id: None,
                        state: rec.state.as_ref().unwrap().to_string(),
                        added_at: *rec.added_at.as_ref().unwrap(),
                    });
                }
                item.history = Some(p);
            }
        }
        Ok(items)
    }

    pub async fn add(
        &self,
        items: Vec<executor::SendedAsyncCommand>,
    ) -> connectors::Result<(errors::ErrorCode, Option<Vec<String>>)> {
        let mut ids = Vec::<String>::new();
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut tx = pool.begin().await?;
        for command in items {
            #[cfg(feature = "postgres")]
            match sqlx::query!(
                r#"INSERT INTO webapi.sended_async_command 
                    ( id, object_type, "service_name", "state", change_state_event ) 
                        VALUES ( $1, $2, $3, $4, $5 )"#,
                command.id,
                command.object_type,
                command.service_name,
                command.state,
                command.change_state_event
            )
            .execute(&mut tx)
            .await
            {
                Ok(_) => ids.push(command.id),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_sended_async_commands db command insert: {}", e);
                    return Ok((errors::ErrorCode::DatabaseError, None));
                }
            };
            #[cfg(feature = "mysql")]
            match sqlx::query(
                r#"INSERT INTO webapi.sended_async_command 
                ( id, object_type, 'service_name', 'state', change_state_event ) 
                    VALUES ( ?, ?, ?, ?, ? )"#,
            )
            .bind(&command.id)
            .bind(&command.object_type)
            .bind(&command.service_name)
            .bind(&command.state)
            .bind(&command.change_state_event)
            .execute(&mut tx)
            .await
            {
                Ok(_) => ids.push(service_name.clone()),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_sended_async_commands db command insert: {}", e);
                    return Ok((errors::ErrorCode::DatabaseError, None));
                }
            };
            if command.history.is_some() {
                for history in &command.history.unwrap() {
                    #[cfg(feature = "postgres")]
                    match sqlx::query!(
                        r#"INSERT INTO webapi.sended_async_command_state_history 
                        ( command_id, "state", added_at ) 
                            VALUES ( $1, $2, $3 )"#,
                        history.command_id,
                        history.state,
                        history.added_at
                    )
                    .execute(&mut tx)
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_sended_async_commands db history insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
                    #[cfg(feature = "mysql")]
                    match sqlx::query(
                        r#"INSERT INTO webapi.sended_async_command_state_history 
                    ( command_id, 'state', added_at ) 
                        VALUES ( ?, ?, ? )"#,
                    )
                    .bind(&history.command_id)
                    .bind(&history.state)
                    .bind(&history.added_at)
                    .execute(&mut tx)
                    .await
                    {
                        Ok(_) => ids.push(service_name.clone()),
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_sended_async_commands db history insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
                }
            }
        }
        match tx.commit().await {
            Ok(_) => {}
            Err(e) => {
                error!("add_sended_async_commands db commit: {}", e);
                return Ok((errors::ErrorCode::DatabaseError, None));
            }
        }
        Ok((errors::ErrorCode::ReplyOk, Some(ids)))
    }

    pub async fn remove(&self, ids: Vec<String>) -> connectors::Result<errors::ErrorCode> {
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut tx = pool.begin().await?;
        match sqlx::query(&self.exp_helper.get_delete_str_exp(
            "webapi.sended_async_command_state_history",
            "command_id",
            &ids,
        ))
        .execute(&mut tx)
        .await
        {
            Ok(_) => {
                match sqlx::query(&self.exp_helper.get_delete_str_exp(
                    "webapi.sended_async_command",
                    "id",
                    &ids,
                ))
                .execute(&mut tx)
                .await
                {
                    Ok(ret) => {
                        if ids.len() == usize::try_from(ret.rows_affected()).unwrap() {
                            match tx.commit().await {
                                Ok(_) => Ok(errors::ErrorCode::ReplyOk),
                                Err(e) => {
                                    error!("remove_sended_async_commands db commit: {}", e);
                                    return Ok(errors::ErrorCode::DatabaseError);
                                }
                            }
                        } else {
                            tx.rollback().await?;
                            Ok(errors::ErrorCode::NotFoundError)
                        }
                    }
                    Err(e) => {
                        error!("remove_sended_async_commands db command delete: {}", e);
                        tx.rollback().await?;
                        Ok(errors::ErrorCode::DatabaseError)
                    }
                }
            }
            Err(e) => {
                error!("remove_sended_async_commands db history delete: {}", e);
                tx.rollback().await?;
                Ok(errors::ErrorCode::DatabaseError)
            }
        }
    }
}

pub struct ReceivedAsyncCommandCollection {
    data_provider: Arc<providers::SqlDbProvider>,
    exp_helper: &'static connectors::ExpHelper,
}

impl ReceivedAsyncCommandCollection {
    pub fn new(
        data_provider: Arc<providers::SqlDbProvider>,
        helper: &'static connectors::ExpHelper,
    ) -> ReceivedAsyncCommandCollection {
        ReceivedAsyncCommandCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get(
        &self,
        ids: Option<Vec<String>>,
    ) -> connectors::Result<Vec<executor::ReceivedAsyncCommand>> {
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut items = Vec::<executor::ReceivedAsyncCommand>::new();
        if ids.is_none() {
            let recs = sqlx::query!(
                r#"SELECT id, object_type, "service_name", request_body, "state", change_state_event,
                    reply_body, proto, added_at, state_changed_at
                        FROM webapi.v_received_async_command"#
            )
            .fetch_all(pool)
            .await?;
            for rec in recs {
                items.push(executor::ReceivedAsyncCommand {
                    id: rec.id.unwrap(),
                    object_type: rec.object_type.unwrap(),
                    service_name: rec.service_name.unwrap(),
                    request_body: rec.request_body.unwrap(),
                    state: rec.state.unwrap(),
                    change_state_event: rec.change_state_event.unwrap(),
                    reply_body: rec.reply_body.unwrap(),
                    proto: rec.proto.unwrap(),
                    added_at: rec.added_at.unwrap(),
                    state_changed_at: rec.state_changed_at.unwrap(),
                    history: None,
                })
            }
        } else {
            let query = self.exp_helper.get_select_str_exp(
                "webapi.v_received_async_command",
                "id",
                &ids.unwrap(),
            );
            let mut cursor = sqlx::query(&query).fetch(pool);
            while let Some(rec) = cursor.try_next().await? {
                items.push(executor::ReceivedAsyncCommand {
                    id: rec.get(0),
                    object_type: rec.get(1),
                    service_name: rec.get(2),
                    request_body: rec.get(3),
                    state: rec.get(4),
                    change_state_event: rec.get(5),
                    reply_body: rec.get(6),
                    proto: rec.get(7),
                    added_at: rec.get(8),
                    state_changed_at: rec.get(9),
                    history: None,
                })
            }
        }
        for mut item in &mut items {
            let recs = sqlx::query!(
                r#"SELECT command_id, "state", added_at
                    FROM webapi.v_received_async_command_state_history
                        WHERE command_id = $1"#,
                &item.id
            )
            .fetch_all(pool)
            .await?;
            if recs.len() > 0 {
                let mut p = Vec::<executor::ReceivedAsyncCommandHistory>::new();
                for rec in recs {
                    p.push(executor::ReceivedAsyncCommandHistory {
                        command_id: None,
                        state: rec.state.as_ref().unwrap().to_string(),
                        added_at: *rec.added_at.as_ref().unwrap(),
                    });
                }
                item.history = Some(p);
            }
        }
        Ok(items)
    }

    pub async fn add(
        &self,
        items: Vec<executor::ReceivedAsyncCommand>,
    ) -> connectors::Result<(errors::ErrorCode, Option<Vec<String>>)> {
        let mut ids = Vec::<String>::new();
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut tx = pool.begin().await?;
        for command in items {
            #[cfg(feature = "postgres")]
            match sqlx::query!(
                r#"INSERT INTO webapi.received_async_command 
                    ( id, object_type, "service_name", request_body, "state", change_state_event, reply_body, proto ) 
                        VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )"#,
                command.id,
                command.object_type,
                command.service_name,
                command.request_body,
                command.state,
                command.change_state_event,
                command.reply_body,
                command.proto
            )
            .execute(&mut tx)
            .await
            {
                Ok(_) => ids.push(command.id),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_received_async_commands db command insert: {}", e);
                    return Ok((errors::ErrorCode::DatabaseError, None));
                }
            };
            #[cfg(feature = "mysql")]
            match sqlx::query(
                r#"INSERT INTO webapi.received_async_command 
                ( id, object_type, "service_name", request_body, "state", change_state_event, reply_body, proto ) 
                    VALUES ( ?, ?, ?, ?, ?, ?, ?, ? )"#,
            )
            .bind(&command.id)
            .bind(&command.object_type)
            .bind(&command.service_name)
            .bind(&command.request_body)
            .bind(&command.state)
            .bind(&command.change_state_event)
            .bind(&command.reply_body)
            .bind(&command.proto)
            .execute(&mut tx)
            .await
            {
                Ok(_) => ids.push(service_name.clone()),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_received_async_commands db command insert: {}", e);
                    return Ok((errors::ErrorCode::DatabaseError, None));
                }
            };
            if command.history.is_some() {
                for history in &command.history.unwrap() {
                    #[cfg(feature = "postgres")]
                    match sqlx::query!(
                        r#"INSERT INTO webapi.received_async_command_state_history 
                        ( command_id, "state", added_at ) 
                            VALUES ( $1, $2, $3 )"#,
                        history.command_id,
                        history.state,
                        history.added_at
                    )
                    .execute(&mut tx)
                    .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_received_async_commands db history insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
                    #[cfg(feature = "mysql")]
                    match sqlx::query(
                        r#"INSERT INTO webapi.received_async_command_state_history 
                    ( command_id, 'state', added_at ) 
                        VALUES ( ?, ?, ? )"#,
                    )
                    .bind(&history.command_id)
                    .bind(&history.state)
                    .bind(&history.added_at)
                    .execute(&mut tx)
                    .await
                    {
                        Ok(_) => ids.push(service_name.clone()),
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_received_async_commands db history insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
                }
            }
        }
        match tx.commit().await {
            Ok(_) => {}
            Err(e) => {
                error!("add_received_async_commands db commit: {}", e);
                return Ok((errors::ErrorCode::DatabaseError, None));
            }
        }
        Ok((errors::ErrorCode::ReplyOk, Some(ids)))
    }

    pub async fn change_state(
        &self,
        state: String,
        ids: Vec<String>,
    ) -> connectors::Result<(
        errors::ErrorCode,
        Option<Vec<(String, executor::AsyncCommandState)>>,
    )> {
        if state == executor::CommandSystemState::Initial.to_string()
            || state == executor::CommandSystemState::Completed.to_string()
        {
            return Err(errors::UnknownAsyncCommandStateError.into());
        }
        // todo: set state for ids receive command, if already set, not update
        // state must be exists in ServiceCommand.state else raise UnknownAsyncCommandStateError
        // add state history
        Ok((errors::ErrorCode::ReplyOk, None))
    }

    // pub async fn complete(
    //     &self,
    //     ids: Vec<String>,
    // ) -> connectors::Result<errors::ErrorCode> {
    // todo: set CommandSystemState.Completed state for ids command, if already set, not update
    // }

    pub async fn remove(&self, ids: Vec<String>) -> connectors::Result<errors::ErrorCode> {
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut tx = pool.begin().await?;
        match sqlx::query(&self.exp_helper.get_delete_str_exp(
            "webapi.received_async_command_state_history",
            "command_id",
            &ids,
        ))
        .execute(&mut tx)
        .await
        {
            Ok(_) => {
                match sqlx::query(&self.exp_helper.get_delete_str_exp(
                    "webapi.received_async_command",
                    "id",
                    &ids,
                ))
                .execute(&mut tx)
                .await
                {
                    Ok(ret) => {
                        if ids.len() == usize::try_from(ret.rows_affected()).unwrap() {
                            match tx.commit().await {
                                Ok(_) => Ok(errors::ErrorCode::ReplyOk),
                                Err(e) => {
                                    error!("remove_received_async_commands db commit: {}", e);
                                    return Ok(errors::ErrorCode::DatabaseError);
                                }
                            }
                        } else {
                            tx.rollback().await?;
                            Ok(errors::ErrorCode::NotFoundError)
                        }
                    }
                    Err(e) => {
                        error!("remove_received_async_commands db command delete: {}", e);
                        tx.rollback().await?;
                        Ok(errors::ErrorCode::DatabaseError)
                    }
                }
            }
            Err(e) => {
                error!("remove_received_async_commands db history delete: {}", e);
                tx.rollback().await?;
                Ok(errors::ErrorCode::DatabaseError)
            }
        }
    }
}
