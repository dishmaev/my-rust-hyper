use super::super::{
    commands, connectors, errors, events, replies, router,
};

pub async fn get(
    dc: &connectors::DataConnector,
    cmd: commands::route::GetRoute,
) -> connectors::Result<replies::route::GetRouteReply> {
    match dc.route.get(cmd.services).await {
        Ok(r) => Ok(replies::route::GetRouteReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            url: None,
            items: Some(r),
        }),
        Err(e) => {
            error!("get_route handler get route collection: {}", e);
            let ec = errors::ErrorCode::DatabaseError;
            Ok(replies::route::GetRouteReply {
                error_code: ec.clone(),
                error_name: Some(dc.error.get(&ec.to_string()).unwrap().clone()),
                url: None,
                items: None,
            })
        }
    }
}

pub async fn get_command(
    dc: &connectors::DataConnector,
    cmd: commands::route::GetServiceCommand,
) -> connectors::Result<replies::route::GetServiceCommandReply> {
    match dc.route.get_command(cmd.services).await {
        Ok(r) => Ok(replies::route::GetServiceCommandReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            url: None,
            items: Some(r),
        }),
        Err(e) => {
            error!("get_route_command handler get route collection: {}", e);
            let ec = errors::ErrorCode::DatabaseError;
            Ok(replies::route::GetServiceCommandReply {
                error_code: ec.clone(),
                error_name: Some(dc.error.get(&ec.to_string()).unwrap().clone()),
                url: None,
                items: None,
            })
        }
    }
}

pub async fn get_event(
    dc: &connectors::DataConnector,
    cmd: commands::route::GetServiceEvent,
) -> connectors::Result<replies::route::GetServiceEventReply> {
    match dc.route.get_event(cmd.services).await {
        Ok(r) => Ok(replies::route::GetServiceEventReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            url: None,
            items: Some(r),
        }),
        Err(e) => {
            error!("get_route_event handler get route collection: {}", e);
            let ec = errors::ErrorCode::DatabaseError;
            Ok(replies::route::GetServiceEventReply {
                error_code: ec.clone(),
                error_name: Some(dc.error.get(&ec.to_string()).unwrap().clone()),
                url: None,
                items: None,
            })
        }
    }
}

pub async fn get_subscription(
    dc: &connectors::DataConnector,
    cmd: commands::route::GetServiceSubscription,
) -> connectors::Result<replies::route::GetServiceSubscriptionReply> {
    match dc.route.get_subscription(cmd.services).await {
        Ok(r) => Ok(replies::route::GetServiceSubscriptionReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            url: None,
            items: Some(r),
        }),
        Err(e) => {
            error!("get_route_subscription handler get route collection: {}", e);
            let ec = errors::ErrorCode::DatabaseError;
            Ok(replies::route::GetServiceSubscriptionReply {
                error_code: ec.clone(),
                error_name: Some(dc.error.get(&ec.to_string()).unwrap().clone()),
                url: None,
                items: None,
            })
        }
    }
}

pub async fn get_service(
    dc: &connectors::DataConnector,
    cmd: commands::route::GetService,
) -> connectors::Result<replies::route::GetServiceReply> {
    match dc.route.get_service(cmd.names).await {
        Ok(r) => Ok(replies::route::GetServiceReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            url: None,
            items: Some(r),
        }),
        Err(e) => {
            error!("get_service handler get service collection: {}", e);
            let ec = errors::ErrorCode::DatabaseError;
            Ok(replies::route::GetServiceReply {
                error_code: ec.clone(),
                error_name: Some(dc.error.get(&ec.to_string()).unwrap().clone()),
                url: None,
                items: None,
            })
        }
    }
}

pub async fn add(
    dc: &connectors::DataConnector,
    cmd: commands::route::AddRoute,
) -> connectors::Result<(
    replies::common::AddStrIdsReply,
    Option<Vec<events::route::OnRouteUpdate>>,
)> {
    let (result, ids) = dc.route.add(cmd.items).await?;
    if result == errors::ErrorCode::ReplyOk {
        let v1 = ids.unwrap();
        let v2 = v1.clone();
        Ok(get_ok_add_str_ids_reply_events!(
            v1,
            Some(vec![events::route::OnRouteUpdate { services: v2 }])
        ))
    } else {
        Ok(get_error_add_str_ids_reply!(&result, dc.error))
    }
}

pub async fn remove(
    dc: &connectors::DataConnector,
    cmd: commands::route::RemoveRoute,
) -> connectors::Result<(
    replies::common::StandardReply,
    Option<Vec<events::route::OnRouteUpdate>>,
)> {
    let result: errors::ErrorCode = dc.route.remove(cmd.services.clone()).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply_events!(Some(vec![
            events::route::OnRouteUpdate { services: cmd.services }
        ])))
    } else {
        Ok(get_error_reply_events!(&result, dc.error))
    }
}

pub async fn on_service_unavailable(
    _dc: &connectors::DataConnector,
    _rt: &router::Router,
    items: Vec<events::route::OnServiceUnavailable>,
) -> connectors::Result<(
    replies::common::StandardReply,
    Option<Vec<events::route::OnRouteUpdate>>,
)> {
    //collect service unavailable info
    if true {
        let mut events = Vec::<events::route::OnRouteUpdate>::new();
        for item in &items {
            events.push(events::route::OnRouteUpdate {
                services: item.services.clone(),
            });
        }
        Ok(get_ok_reply_events!(Some(events)))
    } else {
        Ok(get_ok_reply_events!(None))
    }
}

pub async fn on_route_update(
    dc: &connectors::DataConnector,
    rt: &router::Router,
    _items: Vec<events::route::OnRouteUpdate>,
) -> connectors::Result<replies::common::StandardReply> {
    if rt.is_local {
        let c = dc.route.get_command(None).await?;
        let s = dc.route.get_subscription(None).await?;
        rt.update(c, s).await?;
    } else {
        //get from remote route
    }
    Ok(get_ok_reply!())
}