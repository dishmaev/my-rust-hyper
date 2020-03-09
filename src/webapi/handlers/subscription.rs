use super::super::{connectors, entities::subscription, errors};
use super::models;

pub async fn get(
    dc: &connectors::DataConnector,
    ids: Option<Vec<i32>>,
) -> connectors::Result<Vec<subscription::Subscription>> {
    Ok(dc.subscription.get(ids).await?)
}

pub async fn subscribe(
    dc: &connectors::DataConnector,
    object_name: &str,
    event_name: &str,
    call_back: &str,
) -> connectors::Result<models::Reply> {
    let result = dc
        .subscription
        .unsubscribe(object_name, event_name, call_back)
        .await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply!())
    } else {
        Ok(get_error_reply!(&result, dc.error))
    }
}

pub async fn unsubscribe(
    dc: &connectors::DataConnector,
    object_name: &str,
    event_name: &str,
    call_back: &str,
) -> connectors::Result<models::Reply> {
    let result = dc
        .subscription
        .subscribe(object_name, event_name, call_back)
        .await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply!())
    } else {
        Ok(get_error_reply!(&result, dc.error))
    }
}
