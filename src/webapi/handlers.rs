use super::models;
use hyper::{header, Body, Response, StatusCode};
use serde::ser;
use std::env;
use std::fs;

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

pub async fn spec_json() -> Response<Body> {
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
    .body(Body::from(fs::read_to_string(file.unwrap_or(String::from(DEFAULT_OPENAPI_SPEC))).unwrap()))
    .unwrap()
}

pub async fn spec_yaml() -> Response<Body> {
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
    .body(Body::from(fs::read_to_string(file.unwrap_or(String::from(DEFAULT_OPENAPI_SPEC))).unwrap()))
    .unwrap()
}

pub async fn index() -> Response<Body> {
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

pub async fn signin() -> Response<Body> {
    resp_with_code(StatusCode::OK)
}

pub async fn signup() -> Response<Body> {
    resp_with_code(StatusCode::OK)
}

pub async fn index3(req: models::Event) -> Response<Body> {
    resp(&req)
}

pub async fn index4(req: models::Command) -> Response<Body> {
    if req.name != Some(String::default()) {
        resp(&req)
    } else {
        resp_with_code(StatusCode::BAD_REQUEST)
    }
}
