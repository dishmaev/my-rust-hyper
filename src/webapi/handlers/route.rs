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
) -> connectors::Result<(models::AddStrIdsReply, Option<Vec::<events::route::OnRouteUpdate>>)> {
    let (result, ids) = dc.route.add(items).await?;
    if result == errors::ErrorCode::ReplyOk {
        let v1 = ids.unwrap();
        let v2 = v1.clone();
        Ok(get_ok_add_str_ids_reply_events!(v1, Some(vec![events::route::OnRouteUpdate{services: v2}])))
    } else {
        Ok(get_error_add_str_ids_reply!(&result, dc.error))
    }
}

pub async fn remove(
    dc: &connectors::DataConnector,
    ids: Vec<String>,
) -> connectors::Result<(models::Reply, Option<Vec::<events::route::OnRouteUpdate>>)> {
    let result: errors::ErrorCode = dc.route.remove(ids.clone()).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply_events!(Some(vec![events::route::OnRouteUpdate{services: ids}])))
    } else {
        Ok(get_error_reply_events!(&result, dc.error))
    }
}

pub async fn on_service_unavailable(
    _dc: &connectors::DataConnector,
    _rt: &router::Router,
    _ids: Vec<events::route::OnServiceUnavailable>,
) -> connectors::Result<(models::Reply, Option<Vec::<events::route::OnServiceUnavailable>>)> {
    Ok(get_ok_reply_events!(None))
}

pub async fn on_route_update(
    _dc: &connectors::DataConnector,
    _ep: &publishers::EventPublisher,
    _ce: &executors::CommandExecutor,
    _services: Vec<String>,
) -> connectors::Result<models::Reply> {
    Ok(get_ok_reply!())
}
