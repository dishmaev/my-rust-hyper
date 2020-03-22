use super::super::{connectors, entities::route, errors};
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::Row;
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
        if services.is_none() {
            Ok(sqlx::query_as!(
                route::Command,
                r#"SELECT service_name, priority, object_type, http_to, mq_to
            FROM webapi.v_command"#
            )
            .fetch_all(&mut pool)
            .await?)
        } else {
            let recs = sqlx::query(&self.exp_helper.get_select_str_exp(
                "webapi.v_command",
                "service_name",
                &services.unwrap(),
            ))
            .fetch_all(&mut pool)
            .await?;
            let mut items = Vec::<route::Command>::new();
            for rec in recs {
                items.push(route::Command {
                    service_name: rec.get(0),
                    priority: rec.get(1),
                    object_type: rec.get(2),
                    http_to: rec.get(3),
                    mq_to: rec.get(4),
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
        if services.is_none() {
            Ok(sqlx::query_as!(
                route::Subscription,
                r#"SELECT service_name, object_type, http_to, mq_to
            FROM webapi."v_subscription"
            "#
            )
            .fetch_all(&mut pool)
            .await?)
        } else {
            let recs = sqlx::query(&self.exp_helper.get_select_str_exp(
                "webapi.v_subscription",
                "service_name",
                &services.unwrap(),
            ))
            .fetch_all(&mut pool)
            .await?;
            let mut items = Vec::<route::Subscription>::new();
            for rec in recs {
                items.push(route::Subscription {
                    service_name: rec.get(0),
                    object_type: rec.get(1),
                    http_to: rec.get(2),
                    mq_to: rec.get(3),
                })
            }
            Ok(items)
        }
    }

    pub async fn get(
        &self,
        services: Option<Vec<String>>,
    ) -> connectors::Result<Vec<route::Route>> {
        #[cfg(feature = "postgres")]
        let mut pool: &PgPool = &self.data_provider.pool;
        #[cfg(feature = "mysql")]
        let mut pool: &MySqlPool = &self.data_provider.pool;
        let mut items = Vec::<route::Route>::new();
        let service_recs = if services.is_none() {
            sqlx::query(r#"SELECT "name", "priority", http_helth, mq_helth FROM webapi.service"#)
                .fetch_all(&mut pool)
                .await?
        } else {
            sqlx::query(&self.exp_helper.get_select_str_exp(
                "webapi.service",
                "name",
                &services.unwrap(),
            ))
            .fetch_all(&mut pool)
            .await?
        };
        for service_rec in service_recs {
            let service_name: String = service_rec.get(0);
            let mut commands = Vec::<route::Command>::new();
            let command_recs = sqlx::query(
                r#"SELECT object_type, http_to, mq_to FROM webapi.command WHERE service_name = $1"#,
            )
            .bind(&service_name)
            .fetch_all(&mut pool)
            .await?;
            for command_rec in command_recs {
                commands.push(route::Command {
                    service_name: None,
                    priority: None,
                    object_type: command_rec.get(0),
                    http_to: command_rec.get(1),
                    mq_to: command_rec.get(2),
                });
            }
            let mut subscriptions = Vec::<route::Subscription>::new();
            let subscription_recs = sqlx::query(
                r#"SELECT object_type, http_to, mq_to FROM webapi.subscription WHERE service_name = $1"#,
            )
            .bind(&service_name)
            .fetch_all(&mut pool)
            .await?;
            for subscription_rec in subscription_recs {
                subscriptions.push(route::Subscription {
                    service_name: None,
                    object_type: subscription_rec.get(0),
                    http_to: subscription_rec.get(1),
                    mq_to: subscription_rec.get(2),
                });
            }
            items.push(route::Route {
                service_name: service_name,
                priority: service_rec.get(1),
                http_helth: service_rec.get(2),
                mq_helth: service_rec.get(3),
                command: commands,
                subscription: subscriptions,
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
            #[cfg(feature = "postgres")]
            match sqlx::query!(
                r#"INSERT INTO webapi.service ( "name", priority, http_helth, mq_helth ) VALUES ( $1, $2, $3, $4 )"#,
                route.service_name,
                route.priority,
                route.http_helth,
                route.mq_helth
            )
            .execute(&mut tx)
            .await
            {
                Ok(_) => ids.push(route.service_name.clone()),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_routes db service insert: {}", e);
                    return Ok((errors::ErrorCode::ReplyErrorDatabase, None));
                }
            };
            #[cfg(feature = "mysql")]
            match sqlx::query(
                r#"INSERT INTO webapi.service ( 'name', priority, http_helth, mq_helth ) VALUES ( ?, ?, ?, ? )"#,
            )
            .bind(route.service_name)
            .bind(route.priority)
            .bind(route.http_helth)
            .bind(route.mq_helth)
            .execute(&mut tx)
            .await
            {
                Ok(_) => ids.push(route.service_name.clone()),
                Err(e) => {
                    tx.rollback().await.unwrap();
                    error!("add_routes db service insert: {}", e);
                    return Ok((errors::ErrorCode::ReplyErrorDatabase, None));
                }
            };
            for command in route.command {
                if (command.http_to.is_none() || command.http_to.as_ref().unwrap().is_empty())
                    && (command.mq_to.is_none() || command.mq_to.as_ref().unwrap().is_empty())
                {
                    tx.rollback().await.unwrap();
                    debug!("add_routes db command insert: http_to and mq_to not set");
                    return Err(errors::UnsetRequiredValueError.into());
                }
                #[cfg(feature = "postgres")]
                match sqlx::query!(
                    r#"INSERT INTO webapi.command ( service_name, object_type, http_to, mq_to ) VALUES ( $1, $2, $3, $4 )"#,
                    route.service_name,
                    command.object_type,
                    command.http_to.unwrap_or_default(),
                    command.mq_to.unwrap_or_default()
                )
                .execute(&mut tx)
                .await
                {
                    Ok(_) => {},
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db command insert: {}", e);
                        return Ok((errors::ErrorCode::ReplyErrorDatabase, None));
                    }
                };
                #[cfg(feature = "mysql")]
                match sqlx::query(r#"INSERT INTO webapi.command ( service_name, object_type, http_to, mq_to ) VALUES ( ?, ?, ?, ? )"#)
                    .bind(route.service_name)
                    .bind(command.object_type)
                    .bind(command.http_to.unwrap_or_default())
                    .bind(command.mq_to.unwrap_or_default())
                    .execute(&mut tx)
                    .await
                {
                    Ok(_) => ids.push(route.service_name),
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db command insert: {}", e);
                        return Ok((errors::ErrorCode::ReplyErrorDatabase, None));
                    }
                };
            }
            for subscription in route.subscription {
                if (subscription.http_to.is_none() || subscription.http_to.as_ref().unwrap().is_empty())
                    && (subscription.mq_to.is_none() || subscription.mq_to.as_ref().unwrap().is_empty())
                {
                    tx.rollback().await.unwrap();
                    debug!("add_routes db subscription insert: http_to and mq_to not set");
                    return Err(errors::UnsetRequiredValueError.into());
                }
                #[cfg(feature = "postgres")]
                match sqlx::query!(
                    r#"INSERT INTO webapi.subscription ( service_name, object_type, http_to, mq_to ) VALUES ( $1, $2, $3, $4 )"#,
                    route.service_name,
                    subscription.object_type,
                    subscription.http_to.unwrap_or_default(),
                    subscription.mq_to.unwrap_or_default()
                )
                .execute(&mut tx)
                .await
                {
                    Ok(_) => {},
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db subscription insert: {}", e);
                        return Ok((errors::ErrorCode::ReplyErrorDatabase, None));
                    }
                };
                #[cfg(feature = "mysql")]
                match sqlx::query(r#"INSERT INTO webapi.subscription ( service_name, object_type, http_to, mq_to ) VALUES ( ?, ?, ?, ? )"#)
                    .bind(route.service_name)
                    .bind(subscription.object_type)
                    .bind(subscription.http_to.unwrap_or_default())
                    .bind(subscription.mq_to.unwrap_or_default())
                    .execute(&mut tx)
                    .await
                {
                    Ok(_) => ids.push(route.service_name),
                    Err(e) => {
                        tx.rollback().await.unwrap();
                        error!("add_routes db subscription insert: {}", e);
                        return Ok((errors::ErrorCode::ReplyErrorDatabase, None));
                    }
                };
            }
        }
        match tx.commit().await {
            Ok(_) => {}
            Err(e) => {
                error!("add_routes db commit: {}", e);
                return Ok((errors::ErrorCode::ReplyErrorDatabase, None));
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
            "webapi.subscription",
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
                        match sqlx::query(&self.exp_helper.get_delete_str_exp(
                            "webapi.service",
                            "name",
                            &ids,
                        ))
                        .execute(&mut tx)
                        .await
                        {
                            Ok(ret) => {
                                if ids.len() == usize::try_from(ret).unwrap() {
                                    match tx.commit().await {
                                        Ok(_) => Ok(errors::ErrorCode::ReplyOk),
                                        Err(e) => {
                                            error!("remove_routes db commit: {}", e);
                                            return Ok(errors::ErrorCode::ReplyErrorDatabase);
                                        }
                                    }
                                } else {
                                    tx.rollback().await?;
                                    Ok(errors::ErrorCode::ReplyErrorNotFound)
                                }
                            }
                            Err(e) => {
                                error!("remove_routes db service delete: {}", e);
                                tx.rollback().await?;
                                Ok(errors::ErrorCode::ReplyErrorDatabase)
                            }
                        }
                    }
                    Err(e) => {
                        error!("remove_routes db command delete: {}", e);
                        tx.rollback().await?;
                        Ok(errors::ErrorCode::ReplyErrorDatabase)
                    }
                }
            }
            Err(e) => {
                error!("remove_routes db subscription delete: {}", e);
                tx.rollback().await?;
                Ok(errors::ErrorCode::ReplyErrorDatabase)
            }
        }
    }
}
