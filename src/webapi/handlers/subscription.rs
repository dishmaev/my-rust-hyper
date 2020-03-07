use super::super::{connectors, collections::*, errors, models};

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
    Ok(dc
        .subscription
        .subscribe(object_name, event_name, call_back)
        .await?)
}

pub async fn unsubscribe(
    dc: &connectors::DataConnector,
    object_name: &str,
    event_name: &str,
    call_back: &str,
) -> connectors::Result<models::Reply> {
    Ok(dc
        .subscription
        .unsubscribe(object_name, event_name, call_back)
        .await?)
}
