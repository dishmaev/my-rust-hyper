use super::super::{
    access, commands, connectors, entities::*, events, executors, handlers, publishers, router,
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
        if !params.contains_key("correlation_id") {
            return Ok(resp_with_code(StatusCode::BAD_REQUEST));
        }
        let correlation_id = params.get("correlation_id").unwrap();
        let reply_to = if params.contains_key("reply_to") {
            Some(params.get("reply_to").unwrap())
        } else {
            None
        };
        Ok(match parts.uri.path() {
            path::USR_SIGHN_IN => resp(handlers::usr::signin(&dc).await),
            path::USR_SIGHN_UP => resp(handlers::usr::signup(&dc).await),
            path::ROUTE_GET => {
                let cmd: Option<commands::route::GetRoute> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_route handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_COMMAND_GET => {
                let cmd: Option<commands::route::GetServiceCommand> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get_command(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_route_command handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_EVENT_GET => {
                let cmd: Option<commands::route::GetServiceEvent> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get_event(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_route_event handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_SUBSCIBTION_GET => {
                let cmd: Option<commands::route::GetServiceSubscription> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get_subscription(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_route_subscription handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_SERVICE_GET => {
                let cmd: Option<commands::route::GetService> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get_service(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_service handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_ADD => {
                let cmd: Option<commands::route::AddRoute> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    let res = match handlers::route::add(&dc, cmd.unwrap()).await {
                        Ok(r) => r,
                        Err(e) => {
                            error!("handler: {}", e);
                            return Ok(resp_with_code(StatusCode::INTERNAL_SERVER_ERROR));
                        }
                    };
                    if res.0.is_ok() {
                        match ep.send(correlation_id, res.1.unwrap()).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("event publisher: {}", e);
                                return Ok(resp_with_code(StatusCode::INTERNAL_SERVER_ERROR));
                            }
                        }
                    }
                    resp_event(res.0)
                } else {
                    error!("add_routes handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_REMOVE => {
                let cmd: Option<commands::route::RemoveRoute> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    let res = match handlers::route::remove(&dc, cmd.unwrap()).await {
                        Ok(r) => r,
                        Err(e) => {
                            error!("handler: {}", e);
                            return Ok(resp_with_code(StatusCode::INTERNAL_SERVER_ERROR));
                        }
                    };
                    if res.0.is_ok() {
                        match ep.send(correlation_id, res.1.unwrap()).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("event publisher: {}", e);
                                return Ok(resp_with_code(StatusCode::INTERNAL_SERVER_ERROR));
                            }
                        }
                    }
                    resp_event(res.0)
                } else {
                    error!("remove_routes handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_EVENT_ON_SERVICE_UNAVAILABLE => {
                let events: Option<Vec<events::route::OnServiceUnavailable>> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if events.is_some() {
                    let res =
                        match handlers::route::on_service_unavailable(&dc, &rt, events.unwrap())
                            .await
                        {
                            Ok(r) => r,
                            Err(e) => {
                                error!("handler: {}", e);
                                return Ok(resp_with_code(StatusCode::INTERNAL_SERVER_ERROR));
                            }
                        };
                    if res.0.is_ok() {
                        match ep.send(correlation_id, res.1.unwrap()).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("event publisher: {}", e);
                                return Ok(resp_with_code(StatusCode::INTERNAL_SERVER_ERROR));
                            }
                        }
                    }
                    resp_event(res.0)
                } else {
                    error!("on_service_unavailable handler: bad body");
                    resp_with_code(StatusCode::BAD_REQUEST)
                }
            }
            path::EVENT_ON_ROUTE_UPDATE => {
                let events: Option<Vec<events::route::OnRouteUpdate>> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if events.is_some() {
                    resp(handlers::route::on_route_update(&dc, &rt, events.unwrap()).await)
                } else {
                    error!("on_route_update handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::USR_ITEMS => resp(handlers::usr::get(&dc, None).await),
            path::CAR_GET => {
                let cmd: Option<commands::car::GetCar> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::car::get(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_car handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::CAR_ADD => {
                let cmd: Option<commands::car::AddCar> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::car::add(&dc, cmd.unwrap()).await)
                } else {
                    error!("add_cars handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::CAR_MODIFY => {
                let cmd: Option<commands::car::ModifyCar> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::car::modify(&dc, &ce, cmd.unwrap()).await)
                } else {
                    error!("modify_cars handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::CAR_REMOVE => {
                let cmd: Option<commands::car::RemoveCar> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::car::remove(&dc, cmd.unwrap()).await)
                } else {
                    error!("remove_cars handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::CAR_RESERVE => {
                let cmd: Option<commands::car::ReserveCar> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::car::reserve(&dc, cmd.unwrap()).await)
                } else {
                    error!("reserve_cars handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::SCHEMA => {
                if params.contains_key("object_type") {
                    let ot = params.get("object_type").unwrap().as_str();
                    if rt.schema.contains_key(ot) {
                        resp_schema(&rt.schema.get(ot))
                    } else {
                        error!("schema handler: bad request");
                        return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                    }
                } else {
                    error!("schema handler: bad request");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ERROR => {
                if params.contains_key("error_code") {
                    let ec = params.get("error_code").unwrap().as_str();
                    if dc.error.contains_key(ec) {
                        resp(Ok(error::Error {
                            error_code: ec.to_string(),
                            error_name: dc.error.get(ec).unwrap().clone(),
                        }))
                    } else {
                        error!("error handler: bad body");
                        return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                    }
                } else {
                    error!("error handler: bad request");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::HELTH => return Ok(resp_with_code(StatusCode::OK)),
            _ => resp_with_code(StatusCode::NOT_FOUND),
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

fn resp_event<T>(res: T) -> Response<Body>
where
    T: ser::Serialize,
{
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(serde_json::to_string(&res).unwrap()))
        .unwrap()
}

fn resp_schema<T>(res: T) -> Response<Body>
where
    T: ser::Serialize,
{
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(serde_json::to_string(&res).unwrap()))
        .unwrap()
}

pub fn resp_with_code(status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .unwrap()
}
