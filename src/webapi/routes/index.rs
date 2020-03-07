use hyper::{header, Body, Response, StatusCode};
use std::env;
use std::fs;

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
        .body(Body::from(
            fs::read_to_string(file.unwrap_or(String::from(DEFAULT_OPENAPI_SPEC))).unwrap(),
        ))
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
        .body(Body::from(
            fs::read_to_string(file.unwrap_or(String::from(DEFAULT_OPENAPI_SPEC))).unwrap(),
        ))
        .unwrap()
}

pub async fn handler() -> Response<Body> {
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
