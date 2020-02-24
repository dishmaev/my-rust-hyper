use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::convert::TryFrom;

use super::models;

pub enum ErrorCode {
    ReplyErrorDatabase = -1,
    ReplyErrorNotFound = -100,
}

pub struct ReplyProvider {
    error: HashMap<isize, String>,
}

impl ReplyProvider {
    pub fn from_app_settings(app_settings: &models::AppSettings) -> ReplyProvider {
        let mut e: HashMap<isize, String> = HashMap::new();
        for item in &app_settings.error {
            e.insert(item.code, item.name.clone());
        }
        ReplyProvider { error: e }
    }

    pub fn get_ok_reply(&self) -> Option<models::Reply> {
        Some(models::Reply {
            error_code: 0,
            error_name: None,
        })
    }

    pub fn get_ok_add_reply(&self, ids: Vec<i32>) -> Option<models::AddReply> {
        Some(models::AddReply {
            error_code: 0,
            error_name: None,
            ids: Some(ids),
        })
    }

    pub fn get_error_reply(&self, error_code: ErrorCode) -> Option<models::Reply> {
        let ec = error_code as isize;
        Some(models::Reply {
            error_code: ec,
            error_name: Some(self.error.get(&ec).unwrap().clone()),
        })
    }

    pub fn get_error_add_reply(&self, error_code: ErrorCode) -> Option<models::AddReply> {
        let ec = error_code as isize;
        Some(models::AddReply {
            error_code: ec,
            error_name: Some(self.error.get(&ec).unwrap().clone()),
            ids: None,
        })
    }
}

fn get_ids_as_exp(ids: &Vec<i32>) -> String {
    let mut result: String = String::with_capacity(100);
    for item in ids {
        if result.len() != 0 {
            result.push(',');
        }
        result.push_str(&item.to_string());
    }
    result
}

fn get_delete_in_exp(table: &str, ids: &Vec<i32>) -> String {
    format!(
        "DELETE FROM {} WHERE id IN ({})",
        table,
        get_ids_as_exp(ids)
    )
}

fn get_select_in_exp(table: &str, ids: &Vec<i32>) -> String {
    format!(
        "SELECT * FROM {} WHERE id IN ({})",
        table,
        get_ids_as_exp(ids)
    )
}

pub async fn signin(reply_provider: &ReplyProvider) -> Option<models::Reply> {
    reply_provider.get_ok_reply()
}

pub async fn signup(reply_provider: &ReplyProvider) -> Option<models::Reply> {
    reply_provider.get_ok_reply()
}

pub async fn get_subscriptions(ids: Option<Vec<i8>>) -> Option<Vec<models::Subscription>> {
    let mut items = Vec::<models::Subscription>::new();
    items.push(models::Subscription {
        id: Some(1),
        object_name: Some("car".to_string()),
        event_name: Some("ondelete".to_string()),
        call_back: "http://my.ru".to_string(),
    });
    Some(items)
}

pub async fn subscribe(
    reply_provider: &ReplyProvider,
    object_name: &str,
    event_name: &str,
    call_back: &str,
) -> Option<models::Reply> {
    reply_provider.get_ok_reply()
}

pub async fn unsubscribe(
    reply_provider: &ReplyProvider,
    object_name: &str,
    event_name: &str,
    call_back: &str,
) -> Option<models::Reply> {
    reply_provider.get_ok_reply()
}

pub async fn get_cars(mut pool: &PgPool, ids: Option<Vec<i32>>) -> Option<Vec<models::Car>> {
    if ids.is_none() {
        match sqlx::query_as!(models::Car, r#"SELECT id,car_name FROM public.car"#)
            .fetch_all(&mut pool)
            .await
        {
            Ok(items) => Some(items),
            Err(e) => {
                eprintln!("get_cars handler error: {}", e);
                None
            }
        }
    } else {
        let items = match sqlx::query(&get_select_in_exp("public.car", &ids.unwrap()))
            .fetch_all(&mut pool)
            .await
        {
            Ok(items) => items,
            Err(e) => {
                eprintln!("get_cars handler error: {}", e);
                return None;
            }
        };
        let mut result = Vec::<models::Car>::new();
        for item in items {
            result.push(models::Car {
                id: item.get(0),
                car_name: item.get(1),
            })
        }
        Some(result)
    }
}

pub async fn add_cars(
    pool: &PgPool,
    reply_provider: &ReplyProvider,
    items: Vec<models::Car>,
) -> Option<models::AddReply> {
    let mut ids = Vec::<i32>::new();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("add_cars handler error: {}", e);
            return None;
        }
    };
    for item in items {
        match sqlx::query!(
            r#"INSERT INTO public.car ( car_name ) VALUES ( $1 ) RETURNING id"#,
            item.car_name
        )
        .fetch_one(&mut tx)
        .await
        {
            Ok(rec) => ids.push(rec.id),
            Err(e) => {
                tx.rollback().await.unwrap();
                eprintln!("add_cars handler error: {}", e);
                return reply_provider.get_error_add_reply(ErrorCode::ReplyErrorDatabase);
            }
        };
    }
    match tx.commit().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("add_cars handler error: {}", e);
            return reply_provider.get_error_add_reply(ErrorCode::ReplyErrorDatabase);
        }
    }
    reply_provider.get_ok_add_reply(ids)
}

pub async fn update_cars(
    pool: &PgPool,
    reply_provider: &ReplyProvider,
    items: Vec<models::Car>,
) -> Option<models::Reply> {
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("update_cars handler error: {}", e);
            return None;
        }
    };
    let mut count: u64 = 0;
    for item in &items {
        match sqlx::query!(
            r#"UPDATE public.car SET car_name = $1 WHERE id = $2"#,
            item.car_name,
            item.id.unwrap()
        )
        .execute(&mut tx)
        .await
        {
            Ok(ret) => count += ret,
            Err(e) => {
                eprintln!("update_cars handler error: {}", e);
                match tx.rollback().await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("update_cars handler error: {}", e);
                        return None;
                    }
                };
                return reply_provider.get_error_reply(ErrorCode::ReplyErrorDatabase);
            }
        };
    }
    if items.len() == usize::try_from(count).unwrap() {
        match tx.commit().await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("update_cars handler error: {}", e);
                return reply_provider.get_error_reply(ErrorCode::ReplyErrorDatabase);
            }
        }
        reply_provider.get_ok_reply()
    } else {
        match tx.rollback().await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("update_cars handler error: {}", e);
                return None;
            }
        };
        reply_provider.get_error_reply(ErrorCode::ReplyErrorNotFound)
    }
}

pub async fn delete_cars(
    pool: &PgPool,
    reply_provider: &ReplyProvider,
    ids: Vec<i32>,
) -> Option<models::Reply> {
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("delete_cars handler error: {}", e);
            return None;
        }
    };
    match sqlx::query(&get_delete_in_exp("public.car", &ids))
        .execute(&mut tx)
        .await
    {
        Ok(ret) => {
            if ids.len() == usize::try_from(ret).unwrap() {
                match tx.commit().await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("delete_cars handler error: {}", e);
                        return reply_provider.get_error_reply(ErrorCode::ReplyErrorDatabase);
                    }
                }
                reply_provider.get_ok_reply()
            } else {
                match tx.rollback().await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("delete_cars handler error: {}", e);
                        return None;
                    }
                };
                reply_provider.get_error_reply(ErrorCode::ReplyErrorNotFound)
            }
        }
        Err(e) => {
            eprintln!("delete_cars handler error: {}", e);
            match tx.rollback().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("delete_cars handler error: {}", e);
                    return None;
                }
            };
            reply_provider.get_error_reply(ErrorCode::ReplyErrorDatabase)
        }
    }
}
