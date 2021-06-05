use super::super::{
    access, commands, connectors, errors, events, executors, handlers, publishers,
    router,
};
use super::{index, path};
use hyper::{header, Body, Method, Request, Response, StatusCode};
use bytes::Buf;
use serde::ser;
use std::collections::HashMap;
use std::sync::Arc;
use std::convert::From;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

// pub type Handler = dyn FnOnce(
//     Body,
//     HashMap<String, String>,
// ) -> Box<dyn Future<Output = (Body, HashMap<String, String>)>>;

pub type Handler = fn(Body, HashMap<String, String>) -> (Body, HashMap<String, String>);

// pub async fn async_command_handler(
//     body: Body,
//     param: HashMap<String, String>,
// ) -> (Body, HashMap<String, String>) {
//     (Body::empty(), HashMap::<String, String>::new())
// }

// pub async fn event_handler(
//     body: Body,
//     param: HashMap<String, String>,
// ) -> (Body, HashMap<String, String>) {
//     (Body::empty(), HashMap::<String, String>::new())
// }

pub async fn service_route(
    req: Request<Body>,
    dc: Arc<connectors::DataConnector>,
    ac: Arc<access::AccessChecker>,
    ce: Arc<executors::CommandExecutor>,
    ep: Arc<publishers::EventPublisher>,
    rt: Arc<router::Router>,
    hr: Arc<HashMap<&str, Handler>>,
) -> Result<Response<Body>> {
    if hr.contains_key("s1") {
        let f = hr.get("s1").unwrap();
        let _ = f(Body::empty(), HashMap::<String, String>::new());
    }
    let (parts, body) = req.into_parts();
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
        let _service_name = if params.contains_key("service_name") {
            Some(params.get("service_name").unwrap())
        } else {
            None
        };
        let reader = hyper::body::aggregate(body).await?.reader();
        Ok(match parts.uri.path() {
            path::USR_SIGHN_IN => resp(handlers::usr::signin(&dc).await),
            path::USR_SIGHN_UP => resp(handlers::usr::signup(&dc).await),
            path::ROUTER_ROUTE_GET => {
                let cmd: Option<commands::route::GetRoute> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_route handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTER_COMMAND_GET => {
                let cmd: Option<commands::route::GetServiceCommand> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get_command(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_route_command handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTER_EVENT_GET => {
                let cmd: Option<commands::route::GetServiceEvent> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get_event(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_route_event handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTER_SUBSCIBTION_GET => {
                let cmd: Option<commands::route::GetServiceSubscription> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get_subscription(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_route_subscription handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTER_SERVICE_GET => {
                let cmd: Option<commands::route::GetService> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::route::get_service(&dc, cmd.unwrap()).await)
                } else {
                    error!("get_service handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTER_ROUTE_ADD => {
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
                    resp(Ok(res.0))
                } else {
                    error!("add_routes handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTER_ROUTE_REMOVE => {
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
                    resp(Ok(res.0))
                } else {
                    error!("remove_routes handler: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTER_EVENT_ON_SERVICE_UNAVAILABLE => {
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
                    resp(Ok(res.0))
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
            path::EVENT_ON_ASYNC_COMMAND_STATE_CHANGE => {
                let events: Option<Vec<events::executor::OnAsyncCommandStateChange>> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if events.is_some() {
                    resp(
                        handlers::executor::on_async_command_state_change(
                            &dc,
                            &rt,
                            events.unwrap(),
                        )
                        .await,
                    )
                } else {
                    error!("on_async_command_state_change handler: bad body");
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
            path::CAR_CHANGE => {
                let cmd: Option<commands::car::ChangeCar> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if cmd.is_some() {
                    resp(handlers::car::change(&dc, &ce, cmd.unwrap()).await)
                } else {
                    error!("change_cars handler: bad body");
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
            path::STATE => {
                if params.contains_key("async_command_id") {
                    match ce
                        .get_received_async_command_state(
                            params.get("async_command_id").unwrap().as_str(),
                        )
                        .await
                    {
                        Ok(r) => resp(Ok(r)),
                        Err(e) => {
                            error!("state handler: {}", e);
                            if let Some(_) = e.downcast_ref::<errors::AsyncCommandNotFoundError>() {
                                return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                            } else {
                                return Ok(resp_with_code(StatusCode::INTERNAL_SERVER_ERROR));
                            }
                        }
                    }
                } else {
                    error!("state handler: bad request");
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
                    match handlers::route::get_error(&dc, ec) {
                        Ok(r) => resp(Ok(r)),
                        Err(e) => {
                            error!("error handler: {}", e);
                            return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                        }
                    }
                } else {
                    error!("error handler: bad request");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::HELTH => resp(handlers::route::get_helth()),
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
        Ok(r) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(Body::from(serde_json::to_string(&r).unwrap()))
            .unwrap(),
        Err(e) => {
            error!("handler: {}", e);
            resp_with_code(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
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
