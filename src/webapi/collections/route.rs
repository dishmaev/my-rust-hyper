use super::super::{connectors, entities::route, errors};
#[cfg(feature = "postgres")]
use sqlx::postgres::{PgPool, PgQueryAs};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
use sqlx::{Cursor, Row};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;

pub struct RouteCollection {
    exp_helper: &'static connectors::ExpHelper,
    data_provider: Arc<connectors::SqlDbProvider>,
}

impl RouteCollection {
    pub fn new(
        data_provider: Arc<connectors::SqlDbProvider>,
        helper: &'static connectors::ExpHelper,
    ) -> RouteCollection {
        RouteCollection {
            data_provider: data_provider,
            exp_helper: &helper,
        }
    }

    pub async fn get_command(
        &self,
        services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::ServiceCommand>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        let mut items = Vec::<route::ServiceCommand>::new();
        if services.is_none() {
            let recs = sqlx::query!(
                r#"SELECT service_name, priority, object_type, description, reply_type
            FROM webapi.v_service_command"#,
            )
            .fetch_all(&mut pool)
            .await?;
            for rec in recs {
                items.push(route::ServiceCommand {
                    service_name: rec.service_name,
                    priority: rec.priority,
                    object_type: rec.object_type.unwrap(),
                    description: rec.description.unwrap(),
                    reply_type: rec.reply_type.unwrap(),
                    path: None,
                })
            }
        } else {
            let query = self.exp_helper.get_select_str_exp(
                "webapi.v_service_command",
                "id",
                &services.unwrap(),
            );
            let mut cursor = sqlx::query(&query).fetch(&mut pool);
            while let Some(rec) = cursor.next().await? {
                items.push(route::ServiceCommand {
                    service_name: rec.get(0),
                    priority: rec.get(1),
                    object_type: rec.get(2),
                    description: rec.get(3),
                    reply_type: rec.get(4),
                    path: None,
                })
            }
        }

        let s1 = "s1";
        let s2 = "s2";

        for i in 0..20 {
            let recs = sqlx::query!(
                r#"SELECT proto, "to" FROM webapi.v_service_command_path
                    WHERE service_name = $1 AND object_type = $2"#,
                &s1,
                &s2
            )
            .fetch_all(&mut pool)
            .await?;
        }

        // for item in &items {
        //     let recs = sqlx::query!(
        //         r#"SELECT proto, "to" FROM webapi.v_service_command_path
        //             WHERE service_name = $1 AND object_type = $2"#,
        //         item.service_name.as_ref().unwrap(),
        //         &item.object_type
        //     )
        //     .fetch_all(&mut pool)
        //     .await?;
        //     let mut p = HashMap::<String, String>::new();
        //     for rec in recs {
        //         p.insert(
        //             rec.proto.as_ref().unwrap().to_string(),
        //             rec.to.as_ref().unwrap().to_string(),
        //         );
        //     }
        //     // item.path = Some(p);
        // }
        Ok(items)
    }

    pub async fn get_event(
        &self,
        services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::ServiceEvent>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        if services.is_none() {
            Ok(sqlx::query_as!(
                route::ServiceEvent,
                r#"SELECT service_name, object_type, description
            FROM webapi.v_service_event"#
            )
            .fetch_all(&mut pool)
            .await?)
        } else {
            let query = self.exp_helper.get_select_str_exp(
                "webapi.v_service_event",
                "id",
                &services.unwrap(),
            );
            let items: Vec<route::ServiceEvent> =
                sqlx::query_as(&query).fetch_all(&mut pool).await?;
            Ok(items)
        }
    }

    pub async fn get_subscription(
        &self,
        services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::ServiceSubscription>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        let mut items = Vec::<route::ServiceSubscription>::new();
        if services.is_none() {
            let recs = sqlx::query!(
                r#"SELECT service_name, object_type
            FROM webapi."v_service_subscription"
            "#,
            )
            .fetch_all(&mut pool)
            .await?;
            for rec in recs {
                items.push(route::ServiceSubscription {
                    service_name: rec.service_name,
                    object_type: rec.object_type.unwrap(),
                    path: None,
                })
            }
        } else {
            let query = self.exp_helper.get_select_str_exp(
                "webapi.v_service_subscription",
                "id",
                &services.unwrap(),
            );
            let mut cursor = sqlx::query(&query).fetch(&mut pool);
            while let Some(rec) = cursor.next().await? {
                items.push(route::ServiceSubscription {
                    service_name: rec.get(0),
                    object_type: rec.get(1),
                    path: None,
                })
            }
        }
        for mut item in &mut items {
            let recs = sqlx::query!(
                r#"SELECT proto, "to" FROM webapi.v_service_subscription_path 
                    WHERE service_name = $1 AND object_type = $2"#,
                item.service_name.as_ref().unwrap(),
                &item.object_type
            )
            .fetch_all(&mut pool)
            .await?;
            let mut p = HashMap::<String, String>::new();
            for rec in recs {
                p.insert(
                    rec.proto.as_ref().unwrap().to_string(),
                    rec.to.as_ref().unwrap().to_string(),
                );
            }
            item.path = Some(p);
        }
        Ok(items)
    }

    pub async fn get_service(
        &self,
        services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::Service>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        if services.is_none() {
            Ok(sqlx::query_as!(
                route::Service,
                r#"SELECT name, description, priority
            FROM webapi.v_service"#
            )
            .fetch_all(&mut pool)
            .await?)
        } else {
            let query =
                self.exp_helper
                    .get_select_str_exp("webapi.v_service", "id", &services.unwrap());
            let items: Vec<route::Service> = sqlx::query_as(&query).fetch_all(&mut pool).await?;
            Ok(items)
        }
    }

    pub async fn get(&self, ids: Option<Vec<String>>) -> connectors::Result<Vec<route::Route>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        let mut items = Vec::<route::Route>::new();
        let query =
            self.exp_helper
                .get_select_str_exp("webapi.v_service", "name", &ids.as_ref().unwrap());
        let mut cursor = if ids.is_none() {
            sqlx::query(r#"SELECT "name", "description", "priority" FROM webapi.v_service"#)
                .fetch(&mut pool)
        } else {
            sqlx::query(&query).fetch(&mut pool)
        };
        while let Some(service_rec) = cursor.next().await? {
            let service_name: String = service_rec.get(0);
            let mut commands = Vec::<route::ServiceCommand>::new();
            let command_recs = sqlx::query!(
                r#"SELECT object_type, reply_type, description 
                    FROM webapi.v_service_command WHERE service_name = $1"#,
                &service_name,
            )
            .fetch_all(&mut pool)
            .await?;
            for command_rec in command_recs {
                let recs = sqlx::query!(
                    r#"SELECT proto, "to" 
                        FROM webapi.v_service_command_path 
                            WHERE service_name = $1 AND object_type = $2"#,
                    &service_name,
                    command_rec.object_type.as_ref().unwrap()
                )
                .fetch_all(&mut pool)
                .await?;
                let mut p = HashMap::<String, String>::new();
                for rec in recs {
                    p.insert(rec.proto.unwrap(), rec.to.unwrap());
                }
                commands.push(route::ServiceCommand {
                    service_name: None,
                    priority: None,
                    object_type: command_rec.object_type.unwrap(),
                    reply_type: command_rec.reply_type.unwrap(),
                    description: command_rec.description.unwrap(),
                    path: Some(p),
                });
            }
            let mut events = Vec::<route::ServiceEvent>::new();
            let event_recs = sqlx::query!(
                r#"SELECT object_type, description FROM webapi.v_service_event WHERE service_name = $1"#,
                &service_name
            )
            .fetch_all(&mut pool)
            .await?;
            for event_rec in event_recs {
                events.push(route::ServiceEvent {
                    service_name: None,
                    object_type: event_rec.object_type.unwrap(),
                    description: event_rec.description.unwrap(),
                });
            }
            let mut subscriptions = Vec::<route::ServiceSubscription>::new();
            let subscription_recs = sqlx::query!(
                r#"SELECT object_type FROM webapi.v_service_subscription WHERE service_name = $1"#,
                &service_name
            )
            .fetch_all(&mut pool)
            .await?;
            for subscription_rec in subscription_recs {
                let recs = sqlx::query!(
                    r#"SELECT proto, "to" FROM webapi.v_service_subscription_path 
                        WHERE service_name = $1 AND object_type = $2"#,
                    &service_name,
                    subscription_rec.object_type.as_ref().unwrap()
                )
                .fetch_all(&mut pool)
                .await?;
                let mut p = HashMap::<String, String>::new();
                for rec in recs {
                    p.insert(rec.proto.unwrap(), rec.to.unwrap());
                }
                subscriptions.push(route::ServiceSubscription {
                    service_name: None,
                    object_type: subscription_rec.object_type.unwrap(),
                    path: Some(p),
                });
            }
            let recs = sqlx::query!(
                r#"SELECT proto, helth, schema, reply_to, "error" 
                    FROM webapi.v_service_path WHERE service_name = $1"#,
                &service_name
            )
            .fetch_all(&mut pool)
            .await?;
            let mut p = HashMap::<String, route::ServicePath>::new();
            for rec in recs {
                p.insert(
                    rec.proto.unwrap(),
                    route::ServicePath {
                        helth: rec.helth.unwrap(),
                        schema: rec.schema.unwrap(),
                        reply_to: rec.reply_to.unwrap(),
                        error: rec.error.unwrap(),
                        request: None,
                        event: None,
                    },
                );
            }
            items.push(route::Route {
                service_name: Some(service_name),
                description: service_rec.get(1),
                priority: service_rec.get(2),
                command: commands,
                event: events,
                subscription: subscriptions,
                path: Some(p),
            });
        }
        Ok(items)
    }

    pub async fn add(
        &self,
        items: Vec<route::Route>,
    ) -> connectors::Result<(errors::ErrorCode, Option<Vec<String>>)> {
        let mut ids = Vec::<String>::new();
        #[cfg(feature = "postgres")]
        let pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let pool: &MySqlPool = &self.data_provider.pool;
        let mut tx = pool.begin().await?;
        for route in items {
            let service_name = route.service_name.unwrap();
            #[cfg(feature = "postgres")]
            match sqlx::query!(
                r#"INSERT INTO webapi.service ( "name", description, priority ) VALUES ( $1, $2, $3 )"#,
                service_name,
                route.description,
                route.priority
            )
            .execute(&mut tx)
            .await
            {
                Ok(_) => ids.push(service_name.clone()),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_routes db service insert: {}", e);
                    return Ok((errors::ErrorCode::DatabaseError, None));
                }
            };
            #[cfg(feature = "mysql")]
            match sqlx::query(r#"INSERT INTO webapi.service ( 'name', description, priority ) VALUES ( ?, ?, ? )"#)
                .bind(&service_name)
                .bind(route.description)
                .bind(route.priority)
                .execute(&mut tx)
                .await
            {
                Ok(_) => ids.push(service_name.clone()),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_routes db service insert: {}", e);
                    return Ok((errors::ErrorCode::DatabaseError, None));
                }
            };
            for path in &route.path.unwrap() {
                #[cfg(feature = "postgres")]
                    match sqlx::query!(
                        r#"INSERT INTO webapi.service_path ( "service_name", proto, helth, "schema", "reply_to", "error" ) 
                            VALUES ( $1, $2, $3, $4, $5, $6 )"#,
                        service_name,
                        path.0.clone(),
                        (path.1).helth,
                        (path.1).schema,
                        (path.1).reply_to,
                        (path.1).error
                    )
                    .execute(&mut tx)
                    .await
                    {
                        Ok(_) => ids.push(service_name.clone()),
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_routes db service_path insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
                #[cfg(feature = "mysql")]
                    match sqlx::query(r#"INSERT INTO webapi.service_path ( 'service_name', proto, helth, 'schema', 'reply_to', 'error' ) 
                        VALUES ( ?, ?, ?, ?, ?, ? )"#)
                        .bind(&service_name)
                        .bind(path.0.clone())
                        .bind((path.1).helth)
                        .bind((path.1).schema)
                        .bind((path.1).reply_to)
                        .bind((path.1).error)
                        .execute(&mut tx)
                        .await
                    {
                        Ok(_) => ids.push(service_name.clone()),
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_routes db service_path insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
            }
            for command in route.command {
                #[cfg(feature = "postgres")]
                match sqlx::query!(
                    r#"INSERT INTO webapi.service_command ( service_name, object_type, reply_type, description ) 
                        VALUES ( $1, $2, $3, $4 )"#,
                    service_name,
                    command.object_type,
                    command.reply_type,
                    command.description
                )
                .execute(&mut tx)
                .await
                {
                    Ok(_) => {},
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db command insert: {}", e);
                        return Ok((errors::ErrorCode::DatabaseError, None));
                    }
                };
                #[cfg(feature = "mysql")]
                match sqlx::query(r#"INSERT INTO webapi.service_command ( service_name, object_type, reply_type, description ) 
                    VALUES ( ?, ?, ?, ? )"#)
                    .bind(&service_name)
                    .bind(command.object_type)
                    .bind(command.reply_type)
                    .bind(command.description)
                    .execute(&mut tx)
                    .await
                {
                    Ok(_) => ids.push(service_name.clone()),
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db command insert: {}", e);
                        return Ok((errors::ErrorCode::DatabaseError, None));
                    }
                };
                for path in command.path.unwrap() {
                    #[cfg(feature = "postgres")]
                    match sqlx::query!(
                        r#"INSERT INTO webapi.service_command_path ( "service_name", object_type, proto, "to" ) 
                            VALUES ( $1, $2, $3, $4 )"#,
                        service_name,
                        command.object_type,
                        path.0,
                        path.1
                    )
                    .execute(&mut tx)
                    .await
                    {
                        Ok(_) => ids.push(service_name.clone()),
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_routes db command_path insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
                    #[cfg(feature = "mysql")]
                    match sqlx::query(r#"INSERT INTO webapi.service_command_path ( 'service_name', object_type, proto, 'to' ) 
                        VALUES ( ?, ?, ?, ? )"#)
                        .bind(&service_name)
                        .bind(command.object_type)
                        .bind(path.0)
                        .bind(path.1)
                        .execute(&mut tx)
                        .await
                    {
                        Ok(_) => ids.push(service_name.clone()),
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_routes db command_path insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
                }
            }
            for event in route.event {
                #[cfg(feature = "postgres")]
                match sqlx::query!(
                    r#"INSERT INTO webapi.service_event ( service_name, object_type, description ) 
                        VALUES ( $1, $2, $3 )"#,
                    service_name.clone(),
                    event.object_type,
                    event.description
                )
                .execute(&mut tx)
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db event insert: {}", e);
                        return Ok((errors::ErrorCode::DatabaseError, None));
                    }
                };
                #[cfg(feature = "mysql")]
                match sqlx::query(
                    r#"INSERT INTO webapi.service_event ( service_name, object_type, description ) 
                    VALUES ( ?, ?, ? )"#,
                )
                .bind(route.service_name)
                .bind(event.object_type)
                .bind(event.description)
                .execute(&mut tx)
                .await
                {
                    Ok(_) => ids.push(route.service_name),
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db event insert: {}", e);
                        return Ok((errors::ErrorCode::DatabaseError, None));
                    }
                };
            }
            for subscription in route.subscription {
                #[cfg(feature = "postgres")]
                match sqlx::query!(
                    r#"INSERT INTO webapi.service_subscription ( service_name, object_type ) VALUES ( $1, $2 )"#,
                    service_name.clone(),
                    subscription.object_type
                )
                .execute(&mut tx)
                .await
                {
                    Ok(_) => {},
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db subscription insert: {}", e);
                        return Ok((errors::ErrorCode::DatabaseError, None));
                    }
                };
                #[cfg(feature = "mysql")]
                match sqlx::query(r#"INSERT INTO webapi.service_subscription ( service_name, object_type ) VALUES ( ?, ? )"#)
                    .bind(route.service_name)
                    .bind(subscription.object_type)
                    .execute(&mut tx)
                    .await
                {
                    Ok(_) => ids.push(route.service_name),
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db subscription insert: {}", e);
                        return Ok((errors::ErrorCode::DatabaseError, None));
                    }
                };
                for path in subscription.path.unwrap() {
                    #[cfg(feature = "postgres")]
                    match sqlx::query!(
                        r#"INSERT INTO webapi.service_subscription_path ( "service_name", object_type, proto, "to" ) 
                            VALUES ( $1, $2, $3, $4 )"#,
                        service_name,
                        subscription.object_type,
                        path.0,
                        path.1
                    )
                    .execute(&mut tx)
                    .await
                    {
                        Ok(_) => ids.push(service_name.clone()),
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_routes db subscription_path insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
                    #[cfg(feature = "mysql")]
                    match sqlx::query(r#"INSERT INTO webapi.service_subscription_path ( 'service_name', object_type, proto, 'to' ) 
                        VALUES ( ?, ?, ?, ? )"#)
                        .bind(&service_name)
                        .bind(subscription.object_type)
                        .bind(path.0)
                        .bind(path.1)
                        .execute(&mut tx)
                        .await
                    {
                        Ok(_) => ids.push(service_name.clone()),
                        Err(e) => {
                            tx.rollback().await.unwrap();
                            error!("add_routes db subscription_path insert: {}", e);
                            return Ok((errors::ErrorCode::DatabaseError, None));
                        }
                    };
                }
            }
        }
        match tx.commit().await {
            Ok(_) => {}
            Err(e) => {
                error!("add_routes db commit: {}", e);
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
            "webapi.service_subscription_path",
            "service_name",
            &ids,
        ))
        .execute(&mut tx)
        .await
        {
            Ok(_) => {
                match sqlx::query(&self.exp_helper.get_delete_str_exp(
                    "webapi.service_subscription",
                    "service_name",
                    &ids,
                ))
                .execute(&mut tx)
                .await
                {
                    Ok(_) => {
                        match sqlx::query(&self.exp_helper.get_delete_str_exp(
                            "webapi.service_event",
                            "service_name",
                            &ids,
                        ))
                        .execute(&mut tx)
                        .await
                        {
                            Ok(_) => {
                                match sqlx::query(&self.exp_helper.get_delete_str_exp(
                                    "webapi.service_command_path",
                                    "service_name",
                                    &ids,
                                ))
                                .execute(&mut tx)
                                .await
                                {
                                    Ok(_) => {
                                        match sqlx::query(&self.exp_helper.get_delete_str_exp(
                                            "webapi.service_command",
                                            "service_name",
                                            &ids,
                                        ))
                                        .execute(&mut tx)
                                        .await
                                        {
                                            Ok(_) => {
                                                match sqlx::query(
                                                    &self.exp_helper.get_delete_str_exp(
                                                        "webapi.service_path",
                                                        "service_name",
                                                        &ids,
                                                    ),
                                                )
                                                .execute(&mut tx)
                                                .await
                                                {
                                                    Ok(_) => {
                                                        match sqlx::query(
                                                            &self.exp_helper.get_delete_str_exp(
                                                                "webapi.service",
                                                                "name",
                                                                &ids,
                                                            ),
                                                        )
                                                        .execute(&mut tx)
                                                        .await
                                                        {
                                                            Ok(ret) => {
                                                                if ids.len()
                                                                    == usize::try_from(ret).unwrap()
                                                                {
                                                                    match tx.commit().await {
                                                                Ok(_) => {
                                                                    Ok(errors::ErrorCode::ReplyOk)
                                                                }
                                                                Err(e) => {
                                                                    error!(
                                                                "remove_routes db commit: {}",
                                                                e
                                                            );
                                                                    return Ok(
                                                                errors::ErrorCode::DatabaseError,
                                                            );
                                                                }
                                                            }
                                                                } else {
                                                                    tx.rollback().await?;
                                                                    Ok(errors::ErrorCode::NotFoundError)
                                                                }
                                                            }
                                                            Err(e) => {
                                                                error!(
                                                            "remove_routes db service delete: {}",
                                                            e
                                                        );
                                                                tx.rollback().await?;
                                                                Ok(errors::ErrorCode::DatabaseError)
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        error!("remove_routes db service_path delete: {}", e);
                                                        tx.rollback().await?;
                                                        Ok(errors::ErrorCode::DatabaseError)
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                error!("remove_routes db command delete: {}", e);
                                                tx.rollback().await?;
                                                Ok(errors::ErrorCode::DatabaseError)
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("remove_routes db command_path delete: {}", e);
                                        tx.rollback().await?;
                                        Ok(errors::ErrorCode::DatabaseError)
                                    }
                                }
                            }
                            Err(e) => {
                                error!("remove_routes db event delete: {}", e);
                                tx.rollback().await?;
                                Ok(errors::ErrorCode::DatabaseError)
                            }
                        }
                    }
                    Err(e) => {
                        error!("remove_routes db subscription delete: {}", e);
                        tx.rollback().await?;
                        Ok(errors::ErrorCode::DatabaseError)
                    }
                }
            }
            Err(e) => {
                error!("remove_routes db subscription_path delete: {}", e);
                tx.rollback().await?;
                Ok(errors::ErrorCode::DatabaseError)
            }
        }
    }
}
