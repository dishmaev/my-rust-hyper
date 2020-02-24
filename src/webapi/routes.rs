use super::{handlers, models};
use base64;
use bytes::buf::BufExt;
use hyper::{error::Result, header, Body, Method, Request, Response, StatusCode};
use serde::ser;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::{Arc};
use sqlx::PgPool;

pub const ROUTE_SIGHN_IN: &str = "/api/signin";
pub const ROUTE_SIGHN_UP: &str = "/api/signup";

const ROUTE_SUBSCRIPTION_ITEMS: &str = "/subscriptions";
const ROUTE_SUBSCRIPTION_GET: &str = "/subscription/get";

const ROUTE_CAR_ON_DELETE_SUBSCRIBE: &str = "/car/ondelete/subscribe";
const ROUTE_CAR_ON_DELETE_UNSUBSCRIBE: &str = "/car/ondelete/unsubscribe";

const ROUTE_CAR_ITEMS: &str = "/cars";
const ROUTE_CAR_GET: &str = "/car/get";
const ROUTE_CAR_ADD: &str = "/car/add";
const ROUTE_CAR_UPDATE: &str = "/car/update";
const ROUTE_CAR_DELETE: &str = "/car/delete";

#[cfg(test)]
pub const ROUTES: [&str; 7] = [
    ROUTE_SIGHN_IN,
    ROUTE_SIGHN_UP,
    ROUTE_CAR_GET,
    ROUTE_CAR_DELETE,
    ROUTE_CAR_ON_DELETE_SUBSCRIBE,
    ROUTE_CAR_ON_DELETE_UNSUBSCRIBE,
    ROUTE_SUBSCRIPTION_ITEMS,
];

pub async fn service_route(
    req: Request<Body>,
    pool: Arc<PgPool>,
    access_checker: Arc<AccessChecker>,
    reply_provider: Arc<handlers::ReplyProvider>
) -> Result<Response<Body>> {
    let (parts, body) = req.into_parts();
    let reader = hyper::body::aggregate(body).await?.reader();

    if parts.method == Method::POST {
        let mut is_authorized = false;
        if !parts.headers.get("Authorization").is_none() {
            is_authorized = access_checker.is_authorized(
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
            ROUTE_SIGHN_IN => resp(&handlers::signin(&reply_provider).await),
            ROUTE_SIGHN_UP => resp(&handlers::signup(&reply_provider).await),
            ROUTE_SUBSCRIPTION_ITEMS => resp(&handlers::get_subscriptions(None).await),
            ROUTE_SUBSCRIPTION_GET => resp(
                &handlers::get_subscriptions(serde_json::from_reader(reader).unwrap()).await,
            ),
            ROUTE_CAR_ON_DELETE_SUBSCRIBE => {
                let subscription: models::Subscription = serde_json::from_reader(reader).unwrap();
                resp(
                    &handlers::subscribe(&reply_provider, "car", "ondelete", &subscription.call_back.to_string())
                        .await,
                )
            }
            ROUTE_CAR_ON_DELETE_UNSUBSCRIBE => {
                let subscription: models::Subscription = serde_json::from_reader(reader).unwrap();
                resp(
                    &handlers::unsubscribe(&reply_provider, "car", "ondelete", &subscription.call_back.to_string())
                        .await,
                )
            }
            ROUTE_CAR_ITEMS => resp(&handlers::get_cars(&pool, None).await),
            ROUTE_CAR_GET => {
                resp(&handlers::get_cars(&pool, serde_json::from_reader(reader).unwrap()).await)
            }
            ROUTE_CAR_ADD => {
                resp(&handlers::add_cars(&pool, &reply_provider, serde_json::from_reader(reader).unwrap()).await)
            }
            ROUTE_CAR_UPDATE => {
                resp(&handlers::update_cars(&pool, &reply_provider, serde_json::from_reader(reader).unwrap()).await)
            }
            ROUTE_CAR_DELETE => {
                resp(&handlers::delete_cars(&pool, &reply_provider, serde_json::from_reader(reader).unwrap()).await)
            }
            _ => return Ok(resp_with_code(StatusCode::NOT_FOUND)),
        })
    } else if parts.method == Method::GET {
        Ok(match parts.uri.path() {
            "/" => index().await,
            "/openapi.json" => spec_json().await,
            "/openapi.yaml" => spec_yaml().await,
            _ => resp_with_code(StatusCode::NOT_FOUND),
        })
    } else {
        Ok(resp_with_code(StatusCode::NOT_FOUND))
    }
}

pub fn get_basic_authorization(user: &String, password: &String) -> String {
    format!(
        "Basic {}",
        base64::encode(&format!("{}:{}", user, password))
    )
}

pub struct AccessChecker {
    user_authorization: HashMap<String, String>,
}

impl AccessChecker {
    pub fn from_app_settings(app_settings: &models::AppSettings) -> AccessChecker {
        let mut book_reviews: HashMap<String, String> = HashMap::new();
        for item in &app_settings.authentication {
            book_reviews.insert(
                get_basic_authorization(&item.user, &item.password),
                item.user.clone(),
            );
        }
        AccessChecker {
            user_authorization: book_reviews,
        }
    }

    pub fn is_authorized(&self, header: &str) -> bool {
        *&self.user_authorization.contains_key(header)
    }
}

fn resp<T>(res: &T) -> Response<Body>
where
    T: ser::Serialize,
{
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(serde_json::to_string(&res).unwrap()))
        .unwrap()
}

fn resp_with_code(status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .unwrap()
}

async fn spec_json() -> Response<Body> {
    const ENV_OPENAPI_JSON: &str = "MY_BIN_OPENAPI_JSON";
    const DEFAULT_OPENAPI_SPEC: &str = "openapi.json";
    let file: Option<String> = {
        match env::var(ENV_OPENAPI_JSON).is_ok() {
            true => Some(env::var(ENV_OPENAPI_JSON).unwrap()),
            _ => None,
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(
            fs::read_to_string(file.unwrap_or(String::from(DEFAULT_OPENAPI_SPEC))).unwrap(),
        ))
        .unwrap()
}

async fn spec_yaml() -> Response<Body> {
    const ENV_OPENAPI_YAML: &str = "MY_BIN_OPENAPI_YAML";
    const DEFAULT_OPENAPI_SPEC: &str = "openapi.yaml";
    let file: Option<String> = {
        match env::var(ENV_OPENAPI_YAML).is_ok() {
            true => Some(env::var(ENV_OPENAPI_YAML).unwrap()),
            _ => None,
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .header("Access-Control-Allow-Origin", "https://editor.swagger.io")
        .body(Body::from(
            fs::read_to_string(file.unwrap_or(String::from(DEFAULT_OPENAPI_SPEC))).unwrap(),
        ))
        .unwrap()
}

async fn index() -> Response<Body> {
    const INDEX: &'static str = r#"
    <!doctype html>
    <html>
    <head>
    <title>Microservice</title>
    </head>
    <body>
    <h2>Microservice</h2>
    <p><a href="./openapi.json">openapi.json</a></p> 
    <p><a href="./openapi.yaml">openapi.yaml</a></p> 
    </body>
    </html>
    "#;
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(INDEX))
        .unwrap()
}
