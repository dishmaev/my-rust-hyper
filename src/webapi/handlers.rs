use super::{collections, errors, models};

pub async fn signin() -> collections::Result<collections::Reply> {
    Ok(collections::Reply {
        error_code: errors::ErrorCode::ReplyOk,
        error_name: None,
    })
}

pub async fn signup() -> collections::Result<collections::Reply> {
    Ok(collections::Reply {
        error_code: errors::ErrorCode::ReplyOk,
        error_name: None,
    })
}

pub async fn get_subscriptions(
    ef: &collections::EntityFramework,
    ids: Option<Vec<i32>>,
) -> collections::Result<Vec<models::Subscription>> {
    Ok(ef.subscription_collection.get(&ef.provider, ids).await?)
}

pub async fn subscribe(
    ef: &collections::EntityFramework,
    object_name: &str,
    event_name: &str,
    call_back: &str,
) -> collections::Result<collections::Reply> {
    Ok(ef.subscription_collection.subscribe(&ef.provider, object_name, event_name, call_back).await?)
}

pub async fn unsubscribe(
    ef: &collections::EntityFramework,
    object_name: &str,
    event_name: &str,
    call_back: &str,
) -> collections::Result<collections::Reply> {
    Ok(ef.subscription_collection.unsubscribe(&ef.provider, object_name, event_name, call_back).await?)
}

pub async fn get_cars(
    ef: &collections::EntityFramework,
    ids: Option<Vec<i32>>,
) -> collections::Result<Vec<models::Car>> {
    Ok(ef.car_collection.get(&ef.provider, ids).await?)
}

pub async fn add_cars(
    ef: &collections::EntityFramework,
    items: Vec<models::Car>,
) -> collections::Result<collections::AddReply> {
    Ok(ef.car_collection.add(&ef.provider, items).await?)
}

pub async fn update_cars(
    ef: &collections::EntityFramework,
    items: Vec<models::Car>,
) -> collections::Result<collections::Reply> {
    Ok(ef.car_collection.update(&ef.provider, items).await?)
}

pub async fn delete_cars(
    ef: &collections::EntityFramework,
    ids: Vec<i32>,
) -> collections::Result<collections::Reply> {
    Ok(ef.car_collection.delete(&ef.provider, ids).await?)
}
