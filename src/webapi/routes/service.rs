use super::{index, path};
use super::super::{access, connectors, entities::*, handlers};
use bytes::buf::BufExt;
use hyper::{error::Result, header, Body, Method, Request, Response, StatusCode};
use serde::ser;
use std::sync::Arc;

pub async fn service_route(
    req: Request<Body>,
    dc: Arc<connectors::DataConnector>,
    ac: Arc<access::AccessChecker>,
) -> Result<Response<Body>> {
    let (parts, body) = req.into_parts();
    let reader = hyper::body::aggregate(body).await?.reader();

    if parts.method == Method::POST {
        let mut is_authorized = false;
        if !parts.headers.get("Authorization").is_none() {
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
        Ok(match parts.uri.path() {
            path::ROUTE_SIGHN_IN => resp(handlers::usr::signin(&dc).await),
            path::ROUTE_SIGHN_UP => resp(handlers::usr::signup(&dc).await),
            path::ROUTE_SUBSCRIPTION_ITEMS => resp(handlers::subscription::get(&dc, None).await),
            path::ROUTE_SUBSCRIPTION_GET => resp(
                handlers::subscription::get(&dc, serde_json::from_reader(reader).unwrap()).await,
            ),
            path::ROUTE_CAR_ON_DELETE_SUBSCRIBE => {
                let subscription: subscription::Subscription = serde_json::from_reader(reader).unwrap();
                resp(
                    handlers::subscription::subscribe(
                        &dc,
                        "car",
                        "ondelete",
                        &subscription.call_back.to_string(),
                    )
                    .await,
                )
            }
            path::ROUTE_CAR_ON_DELETE_UNSUBSCRIBE => {
                let subscription: subscription::Subscription = serde_json::from_reader(reader).unwrap();
                resp(
                    handlers::subscription::unsubscribe(
                        &dc,
                        "car",
                        "ondelete",
                        &subscription.call_back.to_string(),
                    )
                    .await,
                )
            }
            path::ROUTE_USR_ITEMS => resp(handlers::usr::get(&dc, None).await),
            path::ROUTE_CAR_ITEMS => resp(handlers::car::get(&dc, None).await),
            path::ROUTE_CAR_GET => {
                let ids: Option<Vec<i32>> = serde_json::from_reader(reader).unwrap_or(None);
                if !ids.is_none() {
                    resp(handlers::car::get(&dc, ids).await)
                } else {
                    eprintln!("get_cars handler error: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_CAR_ADD => {
                let items: Option<Vec<car::Car>> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if !items.is_none() {
                    resp(handlers::car::add(&dc, items.unwrap()).await)
                } else {
                    eprintln!("add_cars handler error: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_CAR_UPDATE => {
                let items: Option<Vec<car::Car>> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if !items.is_none() {
                    resp(handlers::car::update(&dc, items.unwrap()).await)
                } else {
                    eprintln!("update_cars handler error: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
            path::ROUTE_CAR_DELETE => {
                let ids: Option<Vec<i32>> = serde_json::from_reader(reader).unwrap_or(None);
                if !ids.is_none() {
                    resp(handlers::car::delete(&dc, ids.unwrap()).await)
                } else {
                    eprintln!("delete_cars handler error: bad body");
                    return Ok(resp_with_code(StatusCode::BAD_REQUEST));
                }
            }
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
            eprintln!("handler error: {}", e);
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

