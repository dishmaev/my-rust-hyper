use super::super::{connectors, entities::route, errors};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::Row;
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
    ) -> connectors::Result<Vec<route::Command>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        let mut items = Vec::<route::Command>::new();
        if services.is_none() {
            let recs = sqlx::query(
                r#"SELECT service_name, priority, object_type, description, reply_type
            FROM webapi.v_command"#,
            )
            .fetch_all(&mut pool)
            .await?;
            for rec in recs {
                items.push(route::Command {
                    service_name: rec.get(0),
                    priority: rec.get(1),
                    object_type: rec.get(2),
                    description: rec.get(3),
                    reply_type: rec.get(4),
                    path: None,
                })
            }
        } else {
            let recs = sqlx::query(&self.exp_helper.get_select_str_exp(
                "webapi.v_command",
                "service_name",
                &services.unwrap(),
            ))
            .fetch_all(&mut pool)
            .await?;
            for rec in recs {
                items.push(route::Command {
                    service_name: rec.get(0),
                    priority: rec.get(1),
                    object_type: rec.get(2),
                    description: rec.get(3),
                    reply_type: rec.get(4),
                    path: None,
                })
            }
        }
        for mut item in &mut items {
            let recs = sqlx::query(
                r#"SELECT proto, "to" FROM webapi.v_command_path WHERE service_name = $1 AND object_type = $2"#)
            .bind(&item.service_name)
            .bind(&item.object_type)
            .fetch_all(&mut pool)
            .await?;
            let mut p = HashMap::<String, String>::new();
            for rec in recs {
                p.insert(rec.get(0), rec.get(1));
            }
            item.path = Some(p);
        }
        Ok(items)
    }

    pub async fn get_event(
        &self,
        services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::Event>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        if services.is_none() {
            Ok(sqlx::query_as!(
                route::Event,
                r#"SELECT service_name, object_type, description
            FROM webapi.v_event"#
            )
            .fetch_all(&mut pool)
            .await?)
        } else {
            let recs = sqlx::query(&self.exp_helper.get_select_str_exp(
                "webapi.v_event",
                "service_name",
                &services.unwrap(),
            ))
            .fetch_all(&mut pool)
            .await?;
            let mut items = Vec::<route::Event>::new();
            for rec in recs {
                items.push(route::Event {
                    service_name: rec.get(0),
                    object_type: rec.get(1),
                    description: rec.get(2),
                })
            }
            Ok(items)
        }
    }

    pub async fn get_subscription(
        &self,
        services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::Subscription>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        let mut items = Vec::<route::Subscription>::new();
        if services.is_none() {
            let recs = sqlx::query(
                r#"SELECT service_name, object_type
            FROM webapi."v_subscription"
            "#,
            )
            .fetch_all(&mut pool)
            .await?;
            for rec in recs {
                items.push(route::Subscription {
                    service_name: rec.get(0),
                    object_type: rec.get(1),
                    path: None,
                })
            }
        } else {
            let recs = sqlx::query(&self.exp_helper.get_select_str_exp(
                "webapi.v_subscription",
                "service_name",
                &services.unwrap(),
            ))
            .fetch_all(&mut pool)
            .await?;
            for rec in recs {
                items.push(route::Subscription {
                    service_name: rec.get(0),
                    object_type: rec.get(1),
                    path: None,
                })
            }
        }
        for mut item in &mut items {
            let recs = sqlx::query(
                r#"SELECT proto, "to" FROM webapi.v_subscription_path WHERE service_name = $1 AND object_type = $2"#)
            .bind(&item.service_name)
            .bind(&item.object_type)
            .fetch_all(&mut pool)
            .await?;
            let mut p = HashMap::<String, String>::new();
            for rec in recs {
                p.insert(rec.get(0), rec.get(1));
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
            let recs = sqlx::query(&self.exp_helper.get_select_str_exp(
                "webapi.v_service",
                "name",
                &services.unwrap(),
            ))
            .fetch_all(&mut pool)
            .await?;
            let mut items = Vec::<route::Service>::new();
            for rec in recs {
                items.push(route::Service {
                    name: rec.get(0),
                    description: rec.get(1),
                    priority: rec.get(2),
                })
            }
            Ok(items)
        }
    }

    pub async fn get(&self, ids: Option<Vec<String>>) -> connectors::Result<Vec<route::Route>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        let mut items = Vec::<route::Route>::new();
        let service_recs = if ids.is_none() {
            sqlx::query(r#"SELECT "name", "description", "priority" FROM webapi.service"#)
                .fetch_all(&mut pool)
                .await?
        } else {
            sqlx::query(&self.exp_helper.get_select_str_exp(
                "webapi.service",
                "name",
                &ids.unwrap(),
            ))
            .fetch_all(&mut pool)
            .await?
        };
        for service_rec in service_recs {
            let service_name: String = service_rec.get(0);
            let mut commands = Vec::<route::Command>::new();
            let command_recs = sqlx::query(
                r#"SELECT object_type, reply_type, description FROM webapi.command WHERE service_name = $1"#,
            )
            .bind(&service_name)
            .fetch_all(&mut pool)
            .await?;
            for command_rec in command_recs {
                let ot: String = command_rec.get(0);
                let recs = sqlx::query(
                    r#"SELECT proto, "to" FROM webapi.v_command_path WHERE service_name = $1 AND object_type = $2"#)
                .bind(&service_name)
                .bind(&ot)
                .fetch_all(&mut pool)
                .await?;
                let mut p = HashMap::<String, String>::new();
                for rec in recs {
                    p.insert(rec.get(0), rec.get(1));
                }
                commands.push(route::Command {
                    service_name: None,
                    priority: None,
                    object_type: ot,
                    reply_type: command_rec.get(1),
                    description: command_rec.get(2),
                    path: Some(p),
                });
            }
            let mut events = Vec::<route::Event>::new();
            let event_recs = sqlx::query(
                r#"SELECT object_type, description FROM webapi.event WHERE service_name = $1"#,
            )
            .bind(&service_name)
            .fetch_all(&mut pool)
            .await?;
            for event_rec in event_recs {
                events.push(route::Event {
                    service_name: None,
                    object_type: event_rec.get(0),
                    description: event_rec.get(1),
                });
            }
            let mut subscriptions = Vec::<route::Subscription>::new();
            let subscription_recs = sqlx::query(
                r#"SELECT object_type FROM webapi.subscription WHERE service_name = $1"#,
            )
            .bind(&service_name)
            .fetch_all(&mut pool)
            .await?;
            for subscription_rec in subscription_recs {
                let ot: String = subscription_rec.get(0);
                let recs = sqlx::query(
                    r#"SELECT proto, "to" FROM webapi.v_subscription_path WHERE service_name = $1 AND object_type = $2"#)
                .bind(&service_name)
                .bind(&ot)
                .fetch_all(&mut pool)
                .await?;
                let mut p = HashMap::<String, String>::new();
                for rec in recs {
                    p.insert(rec.get(0), rec.get(1));
                }
                subscriptions.push(route::Subscription {
                    service_name: None,
                    object_type: ot,
                    path: Some(p),
                });
            }
            let recs = sqlx::query(
                r#"SELECT proto, helth, schema, reply_to, "error" FROM webapi.v_service_path WHERE service_name = $1"#,
            )
            .bind(&service_name)
            .fetch_all(&mut pool)
            .await?;
            let mut p = HashMap::<String, route::ServicePath>::new();
            for rec in recs {
                p.insert(
                    rec.get(0),
                    route::ServicePath {
                        helth: rec.get(1),
                        schema: rec.get(2),
                        reply_to: rec.get(3),
                        error: rec.get(4),
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

    fn get_new_path(
        &self,
        path: &Option<HashMap<String, route::ServicePath>>,
    ) -> HashMap<String, String> {
        let mut hm = HashMap::<String, String>::new();
        hm
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
            let mut new_command_path = HashMap::<String, String>::new();
            let mut new_subscription_path = HashMap::<String, String>::new();
            for path in &route.path.unwrap() {
                new_command_path.insert(
                    path.0.clone(),
                    (path.1).request.as_ref().unwrap().to_string()
                );
                new_subscription_path
                    .insert(path.0.clone(), (path.1).event.as_ref().unwrap().to_string());
                #[cfg(feature = "postgres")]
                    match sqlx::query!(
                        r#"INSERT INTO webapi.service_path ( "service_name", proto, helth, "schema", "reply_to", "error" ) VALUES ( $1, $2, $3, $4, $5, $6 )"#,
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
                    match sqlx::query(r#"INSERT INTO webapi.service_path ( 'service_name', proto, helth, 'schema', 'reply_to', 'error' ) VALUES ( ?, ?, ?, ?, ?, ? )"#)
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
                    r#"INSERT INTO webapi.command ( service_name, object_type, reply_type, description ) VALUES ( $1, $2, $3, $4 )"#,
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
                match sqlx::query(r#"INSERT INTO webapi.command ( service_name, object_type, reply_type, description ) VALUES ( ?, ?, ?, ? )"#)
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
                let paths = if command.path.is_some() {
                    command.path.unwrap()
                } else {
                    new_command_path.clone()
                };
                for path in paths {
                    #[cfg(feature = "postgres")]
                    match sqlx::query!(
                        r#"INSERT INTO webapi.command_path ( "service_name", object_type, proto, "to" ) VALUES ( $1, $2, $3, $4 )"#,
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
                    match sqlx::query(r#"INSERT INTO webapi.command_path ( 'service_name', object_type, proto, 'to' ) VALUES ( ?, ?, ?, ? )"#)
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
                    r#"INSERT INTO webapi.event ( service_name, object_type, description ) VALUES ( $1, $2, $3 )"#,
                    service_name.clone(),
                    event.object_type,
                    event.description
                )
                .execute(&mut tx)
                .await
                {
                    Ok(_) => {},
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db event insert: {}", e);
                        return Ok((errors::ErrorCode::DatabaseError, None));
                    }
                };
                #[cfg(feature = "mysql")]
                match sqlx::query(r#"INSERT INTO webapi.event ( service_name, object_type, description ) VALUES ( ?, ?, ? )"#)
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
                    r#"INSERT INTO webapi.subscription ( service_name, object_type ) VALUES ( $1, $2 )"#,
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
                match sqlx::query(r#"INSERT INTO webapi.subscription ( service_name, object_type ) VALUES ( ?, ? )"#)
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
                let paths = if subscription.path.is_some() {
                    subscription.path.unwrap()
                } else {
                    new_subscription_path.clone()
                };
                for path in paths {
                    #[cfg(feature = "postgres")]
                    match sqlx::query!(
                        r#"INSERT INTO webapi.subscription_path ( "service_name", object_type, proto, "to" ) VALUES ( $1, $2, $3, $4 )"#,
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
                    match sqlx::query(r#"INSERT INTO webapi.subscription_path ( 'service_name', object_type, proto, 'to' ) VALUES ( ?, ?, ?, ? )"#)
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
            "webapi.subscription_path",
            "service_name",
            &ids,
        ))
        .execute(&mut tx)
        .await
        {
            Ok(_) => {
                match sqlx::query(&self.exp_helper.get_delete_str_exp(
                    "webapi.subscription",
                    "service_name",
                    &ids,
                ))
                .execute(&mut tx)
                .await
                {
                    Ok(_) => {
                        match sqlx::query(&self.exp_helper.get_delete_str_exp(
                            "webapi.event",
                            "service_name",
                            &ids,
                        ))
                        .execute(&mut tx)
                        .await
                        {
                            Ok(_) => {
                                match sqlx::query(&self.exp_helper.get_delete_str_exp(
                                    "webapi.command_path",
                                    "service_name",
                                    &ids,
                                ))
                                .execute(&mut tx)
                                .await
                                {
                                    Ok(_) => {
                                        match sqlx::query(&self.exp_helper.get_delete_str_exp(
                                            "webapi.command",
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
