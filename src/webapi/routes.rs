use super::{handlers, models};
use base64;
use bytes::buf::BufExt;
use hyper::{error::Result, Body, Method, Request, Response, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;

pub const ROUTE_PATH_INDEX: &str = "/";
pub const ROUTE_PATH_SPEC_JSON: &str = "/openapi.json";
pub const ROUTE_PATH_SPEC_YAML: &str = "/openapi.yaml";
pub const ROUTE_PATH_SIGHN_IN: &str = "/api/signin";
pub const ROUTE_PATH_SIGHN_UP: &str = "/api/signup";
const PATH_3: &str = "/3";
const PATH_4: &str = "/4";

#[cfg(test)]
pub const ROUTES: [&str; 4] = [ROUTE_PATH_SIGHN_IN, ROUTE_PATH_SIGHN_UP, PATH_3, PATH_4];

pub async fn service_route(
    req: Request<Body>,
    access_checker: Arc<AccessChecker>,
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
    }

    let resp = {
        match (parts.method, parts.uri.path()) {
            (Method::GET, ROUTE_PATH_INDEX) => handlers::index().await,
            (Method::GET, ROUTE_PATH_SPEC_JSON) => handlers::spec_json().await,
            (Method::GET, ROUTE_PATH_SPEC_YAML) => handlers::spec_yaml().await,
            (Method::POST, ROUTE_PATH_SIGHN_IN) => handlers::signin().await,
            (Method::POST, ROUTE_PATH_SIGHN_UP) => handlers::signup().await,
            (Method::POST, PATH_3) => {
                handlers::index3(serde_json::from_reader(reader).unwrap()).await
            }
            (Method::POST, PATH_4) => {
                handlers::index4(serde_json::from_reader(reader).unwrap()).await
            }
            _ => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap(),
        }
    };
    Ok(resp)
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
        let mut book_reviews = HashMap::new();
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
