use super::super::{
    access, connectors, entities::*, events, executors, handlers, publishers, router,
};
use super::{index, path};
use bytes::buf::BufExt;
use hyper::{error::Result, header, Body, Method, Request, Response, StatusCode};
use serde::ser;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn service_route(
    req: Request<Body>,
    dc: Arc<connectors::DataConnector>,
    ac: Arc<access::AccessChecker>,
    ce: Arc<executors::CommandExecutor>,
    ep: Arc<publishers::EventPublisher>,
    rt: Arc<router::Router>,
) -> Result<Response<Body>> {
    let (parts, body) = req.into_parts();
    let reader = hyper::body::aggregate(body).await?.reader();
    if parts.method == Method::POST {
        let mut is_authorized = false;
        if parts.headers.get("Authorization").is_some() {
            is_authorized = ac.is_authorized_by_header(
                parts
                    .headers
                    .get("Authorization")
                    .unwrap()
                    .to_str()
                    .unwrap(),
            );
        }
        if !is_authorized {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("WWW-Authenticate", "Basic realm=\"Access to microservice\"")
                .body(Body::empty())
                .unwrap());
        }
        let params: HashMap<String, String> = parts
            .uri
            .query()
            .map(|v| {
                url::form_urlencoded::parse(v.as_bytes())
                    .into_owned()
                    .collect()
            })
            .unwrap_or_else(HashMap::new);
        if !params.contains_key("CorrelationId") {
            return Ok(resp_with_code(StatusCode::BAD_REQUEST));
        }
        let correlation_id = params.get("CorrelationId");
        Ok(match parts.uri.path() {
            path::ERROR_ITEMS => resp(dc.get_errors(None)),
            path::ERROR_GET => resp(dc.get_errors(serde_json::from_reader(reader).unwrap())),
            path::USR_SIGHN_IN => resp(handlers::usr::signin(&dc).await),
            path::USR_SIGHN_UP => resp(handlers::usr::signup(&dc).await),
            path::ROUTE_ITEMS => resp(handlers::route::get(&dc, None).await),
            path::ROUTE_COMMAND_ITEMS => resp(handlers::route::get_command(&dc, None).await),
            path::ROUTE_SUBSCIBTION_ITEMS => {
                resp(handlers::route::get_subscription(&dc, None).await)
            }
            path::ROUTE_GET => {
                let services: Option<Vec<String>> = serde_json::from_reader(reader).unwrap_or(None);
                if services.is_some() {
                    resp(handlers::route::get(&dc, services).await)
                } else {
                    error!("get_routes handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_COMMAND_GET => {
                let services: Option<Vec<String>> = serde_json::from_reader(reader).unwrap_or(None);
                if services.is_some() {
                    resp(handlers::route::get_command(&dc, services).await)
                } else {
                    error!("get_route_commands handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_SUBSCIBTION_GET => {
                let services: Option<Vec<String>> = serde_json::from_reader(reader).unwrap_or(None);
                if services.is_some() {
                    resp(handlers::route::get_subscription(&dc, services).await)
                } else {
                    error!("get_route_subscriptions handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_ADD => {
                let items: Option<Vec<route::Route>> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if items.is_some() {
                    let res = handlers::route::add(&dc, items.unwrap()).await;
                    if res.as_ref().unwrap().is_ok() {
                        ep.as_ref().send(res.as_ref().unwrap().get_ids());
                    }
                    resp(res)
                } else {
                    error!("add_routes handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_REMOVE => {
                let ids: Option<Vec<String>> = serde_json::from_reader(reader).unwrap_or(None);
                if ids.is_some() {
                    let ids_for_event = ids.clone().unwrap();
                    let res = handlers::route::remove(&dc, ids.unwrap()).await;
                    if res.as_ref().unwrap().is_ok() {
                        ep.as_ref().send(ids_for_event);
                    }
                    resp(res)
                } else {
                    error!("remove_routes handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_EVENT_ON_SERVICE_UNAVAILABLE => {
                let items: Option<Vec<events::route::OnServiceUnavailable>> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if items.is_some() {
                    resp(handlers::route::on_service_unavailable(&dc, &rt, items.unwrap()).await)
                } else {
                    error!("on_service_unavailable handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_EVENT_ON_ROUTE_UPDATE => {
                let items: Option<Vec<String>> = serde_json::from_reader(reader).unwrap_or(None);
                if items.is_some() {
                    resp(handlers::route::on_route_update(&dc, &ce, items.unwrap()).await)
                } else {
                    error!("on_route_update handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::USR_ITEMS => resp(handlers::usr::get(&dc, None).await),
            path::CAR_ITEMS => resp(handlers::car::get(&dc, None).await),
            path::CAR_GET => {
                let ids: Option<Vec<i32>> = serde_json::from_reader(reader).unwrap_or(None);
                if ids.is_some() {
                    resp(handlers::car::get(&dc, ids).await)
                } else {
                    error!("get_cars handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::CAR_ADD => {
                let items: Option<Vec<car::Car>> = serde_json::from_reader(reader).unwrap_or(None);
                if items.is_some() {
                    resp(handlers::car::add(&dc, items.unwrap()).await)
                } else {
                    error!("add_cars handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::CAR_UPDATE => {
                let items: Option<Vec<car::Car>> = serde_json::from_reader(reader).unwrap_or(None);
                if items.is_some() {
                    resp(handlers::car::update(&dc, &ce, items.unwrap()).await)
                } else {
                    error!("update_cars handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::CAR_REMOVE => {
                let ids: Option<Vec<i32>> = serde_json::from_reader(reader).unwrap_or(None);
                if ids.is_some() {
                    resp(handlers::car::remove(&dc, ids.unwrap()).await)
                } else {
                    error!("remove_cars handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::HELTH => return Ok(resp_with_code(StatusCode::OK)),
            _ => return Ok(resp_with_code(StatusCode::NOT_FOUND)),
        })
    } else if parts.method == Method::GET {
        Ok(match parts.uri.path() {
            "/" => index::handler().await,
            "/openapi.json" => index::spec_json().await,
            "/openapi.yaml" => index::spec_yaml().await,
            _ => resp_with_code(StatusCode::NOT_FOUND),
        })
    } else {
        Ok(resp_with_code(StatusCode::NOT_FOUND))
    }
}

fn resp<T>(res: connectors::Result<T>) -> Response<Body>
where
    T: ser::Serialize,
{
    match res {
        Ok(items) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(Body::from(serde_json::to_string(&items).unwrap()))
            .unwrap(),
        Err(e) => {
            error!("handler: {}", e);
            resp_with_code(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn resp_with_code(status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .unwrap()
}
