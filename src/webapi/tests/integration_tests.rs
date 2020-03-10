use super::super::{access, connectors, settings, routes::*};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Method, Request, Response, Server, StatusCode};
use rand::prelude::Rng;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

fn get_basic_authorization(user: &String, password: &String) -> String {
    format!(
        "Basic {}",
        base64::encode(&format!("{}:{}", user, password))
    )
}

async fn get_settings() -> (SocketAddr, Arc<connectors::DataConnector>, Arc<access::AccessChecker>) {
    let mut rng = rand::thread_rng();
    let app_settings: settings::AppSettings =
        serde_json::from_str(&fs::read_to_string("appsettings.test.json").unwrap()).unwrap();
    let data_connector =
        connectors::DataConnector::new(app_settings._error, Some(app_settings._pg_db), Some(app_settings._my_sql_db))
            .await
            .expect("error while initialize data connector");
    let access_checker = access::AccessChecker::_from_app_settings(&app_settings._access.unwrap())
            .await
            .expect("error while initialize access checker");
    (
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), rng.gen_range(15000, 25000)),
        Arc::new(data_connector),
        Arc::new(access_checker),
    )
}

async fn call_service(method: hyper::Method, port: u16, path: &str, body: Body) -> Response<Body> {
    let req = Request::builder()
        .method(method)
        .uri(format!("http://{}:{}{}", Ipv4Addr::LOCALHOST, port, path))
        .header(
            "Authorization",
            get_basic_authorization(&"test".to_string(), &"1234567890".to_string()),
        )
        .body(body)
        .expect("request builder");
    let client = Client::new();
    client.request(req).await.unwrap()
}

#[tokio::test(threaded_scheduler)]
async fn test_index_ok() {
    let (addr, data_connector_arc, access_checker_arc) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        async move { Ok::<_, Error>(service_fn(move |req| service::service_route(req, dc.clone(), ac.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    let resp = call_service(
        Method::GET,
        addr.port(),
        "/",
        Body::empty(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_spec_json_ok() {
    let (addr, data_connector_arc, access_checker_arc) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        async move { Ok::<_, Error>(service_fn(move |req| service::service_route(req, dc.clone(), ac.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    let resp = call_service(
        Method::GET,
        addr.port(),
        "/openapi.json",
        Body::empty(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_spec_yaml_ok() {
    let (addr, data_connector_arc, access_checker_arc) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        async move { Ok::<_, Error>(service_fn(move |req| service::service_route(req, dc.clone(), ac.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    let resp = call_service(
        Method::GET,
        addr.port(),
        "/openapi.yaml",
        Body::empty(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_route_ok() {
    let (addr, data_connector_arc, access_checker_arc) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        async move { Ok::<_, Error>(service_fn(move |req| service::service_route(req, dc.clone(), ac.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    for route in path::ROUTE_WITH_EMPTY_BODY.iter() {
        let resp = call_service(Method::POST, addr.port(), route, Body::empty()).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

#[tokio::test(threaded_scheduler)]
async fn test_route_err() {
    let (addr, data_connector_arc, access_checker_arc) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        async move { Ok::<_, Error>(service_fn(move |req| service::service_route(req, dc.clone(), ac.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    let resp = call_service(Method::POST, addr.port(), "/fake", Body::from("{}")).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}