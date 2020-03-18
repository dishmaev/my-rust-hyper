use super::super::{connectors, entities, errors, events, executors, publishers, router};
use super::models;

pub async fn get(
    dc: &connectors::DataConnector,
    services: Option<Vec<String>>,
) -> connectors::Result<Vec<entities::route::Route>> {
    Ok(dc.route.get(services).await?)
}

pub async fn get_command(
    dc: &connectors::DataConnector,
    services: Option<Vec<String>>,
) -> connectors::Result<Vec<entities::route::Command>> {
    Ok(dc.route.get_command(services).await?)
}

pub async fn get_subscription(
    dc: &connectors::DataConnector,
    services: Option<Vec<String>>,
) -> connectors::Result<Vec<entities::route::Subscription>> {
    Ok(dc.route.get_subscription(services).await?)
}

pub async fn add(
    dc: &connectors::DataConnector,
    items: Vec<entities::route::Route>,
) -> connectors::Result<models::AddStrIdsReply> {
    let (result, ids) = dc.route.add(items).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_add_str_ids_reply!(ids.unwrap()))
    } else {
        Ok(get_error_add_str_ids_reply!(&result, dc.error))
    }
}

pub async fn remove(
    dc: &connectors::DataConnector,
    ids: Vec<String>,
) -> connectors::Result<models::Reply> {
    let result: errors::ErrorCode = dc.route.remove(ids).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply!())
    } else {
        Ok(get_error_reply!(&result, dc.error))
    }
}

pub async fn on_service_unavailable(
    dc: &connectors::DataConnector,
    rt: &router::Router,
    ids: Vec<events::route::OnServiceUnavailable>,
) -> connectors::Result<models::Reply> {
    Ok(get_ok_reply!())
}

pub async fn on_route_update(
    dc: &connectors::DataConnector,
    ce: &executors::CommandExecutor,
    services: Vec<String>,
) -> connectors::Result<models::Reply> {
    Ok(get_ok_reply!())
}
