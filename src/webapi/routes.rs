use super::{handlers};
use bytes::buf::BufExt;
use hyper::{error::Result, Body, Method, Request, Response, StatusCode};

pub const ROUTE_PATH_SPEC_JSON: &str = "/openapi.json";
pub const ROUTE_PATH_SPEC_YAML: &str = "/openapi.yaml";
pub const ROUTE_PATH_SIGHN_IN: &str = "/api/signin";
pub const ROUTE_PATH_SIGHN_UP: &str = "/api/signup";
const ROUTE_PATH_INDEX: &str = "/";
const PATH_3: &str = "/3";
const PATH_4: &str = "/4";

#[cfg(test)]
pub const ROUTES: [&str; 4] = [ROUTE_PATH_SIGHN_IN, ROUTE_PATH_SIGHN_UP, PATH_3, PATH_4];

pub async fn service_route(req: Request<Body>) -> Result<Response<Body>> {
    let (parts, body) = req.into_parts();
    let reader = hyper::body::aggregate(body).await?.reader();

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
