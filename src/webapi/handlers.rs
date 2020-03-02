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
    dc: &collections::DataConnector,
    ids: Option<Vec<i32>>,
) -> collections::Result<Vec<models::Subscription>> {
    Ok(dc.subscription.get(ids).await?)
}

pub async fn subscribe(
    dc: &collections::DataConnector,
    object_name: &str,
    event_name: &str,
    call_back: &str,
) -> collections::Result<collections::Reply> {
    Ok(dc
        .subscription
        .subscribe(object_name, event_name, call_back)
        .await?)
}

pub async fn unsubscribe(
    dc: &collections::DataConnector,
    object_name: &str,
    event_name: &str,
    call_back: &str,
) -> collections::Result<collections::Reply> {
    Ok(dc
        .subscription
        .unsubscribe(object_name, event_name, call_back)
        .await?)
}

pub async fn get_usrs(
    dc: &collections::DataConnector,
    ids: Option<Vec<i32>>,
) -> collections::Result<Vec<models::Usr>> {
    Ok(dc.usr.get(ids).await?)
}

pub async fn get_cars(
    dc: &collections::DataConnector,
    ids: Option<Vec<i32>>,
) -> collections::Result<Vec<models::Car>> {
    Ok(dc.car.get(ids).await?)
}

pub async fn add_cars(
    dc: &collections::DataConnector,
    items: Vec<models::Car>,
) -> collections::Result<collections::AddReply> {
    Ok(dc.car.add(items).await?)
}

pub async fn update_cars(
    dc: &collections::DataConnector,
    items: Vec<models::Car>,
) -> collections::Result<collections::Reply> {
    Ok(dc.car.update(items).await?)
}

pub async fn delete_cars(
    dc: &collections::DataConnector,
    ids: Vec<i32>,
) -> collections::Result<collections::Reply> {
    Ok(dc.car.delete(ids).await?)
}
