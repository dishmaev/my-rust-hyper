use super::super::{models, routes};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Method, Request, Response, Server, StatusCode};
use rand::prelude::Rng;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

fn get_sock_addr_and_app_settings() -> (SocketAddr, Arc<routes::AccessChecker>) {
    const APP_SETTINGS_FILE: &str = "appsettings.test.json";
    let mut rng = rand::thread_rng();
    let config_file: models::AppSettings =
        serde_json::from_str(&fs::read_to_string(APP_SETTINGS_FILE).unwrap()).unwrap();
    let access_checker = routes::AccessChecker {
        user_password: config_file.authentication,
    };
    access_checker.initialize();
    (
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), rng.gen_range(15000, 25000)),
        Arc::new(access_checker),
    )
}

async fn call_service(method: hyper::Method, port: u16, path: &str, body: Body) -> Response<Body> {
    let req = Request::builder()
        .method(method)
        .uri(format!("http://{}:{}{}", Ipv4Addr::LOCALHOST, port, path))
        .header(
            "Authorization",
            routes::get_basic_authorization(&"test".to_string(), &"1234567890".to_string()),
        )
        .body(body)
        .expect("request builder");
    let client = Client::new();
    client.request(req).await.unwrap()
}

#[tokio::test(threaded_scheduler)]
async fn test_index_ok() {
    let (addr, sets) = get_sock_addr_and_app_settings();
    let make_svc = make_service_fn(move |_| {
        let s = sets.clone();
        async move { Ok::<_, Error>(service_fn(move |req| routes::service_route(req, s.clone()))) }
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
        routes::ROUTE_PATH_INDEX,
        Body::empty(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_spec_json_ok() {
    let (addr, sets) = get_sock_addr_and_app_settings();
    let make_svc = make_service_fn(move |_| {
        let s = sets.clone();
        async move { Ok::<_, Error>(service_fn(move |req| routes::service_route(req, s.clone()))) }
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
        routes::ROUTE_PATH_SPEC_JSON,
        Body::empty(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_spec_yaml_ok() {
    let (addr, sets) = get_sock_addr_and_app_settings();
    let make_svc = make_service_fn(move |_| {
        let s = sets.clone();
        async move { Ok::<_, Error>(service_fn(move |req| routes::service_route(req, s.clone()))) }
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
        routes::ROUTE_PATH_SPEC_YAML,
        Body::empty(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_route_ok() {
    let (addr, sets) = get_sock_addr_and_app_settings();
    let make_svc = make_service_fn(move |_| {
        let s = sets.clone();
        async move { Ok::<_, Error>(service_fn(move |req| routes::service_route(req, s.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    for route in routes::ROUTES.iter() {
        let resp = call_service(Method::POST, addr.port(), route, Body::from("{}")).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

#[tokio::test(threaded_scheduler)]
async fn test_route_err() {
    let (addr, sets) = get_sock_addr_and_app_settings();
    let make_svc = make_service_fn(move |_| {
        let s = sets.clone();
        async move { Ok::<_, Error>(service_fn(move |req| routes::service_route(req, s.clone()))) }
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

#[tokio::test(threaded_scheduler)]
async fn test_signin_ok() {
    let (addr, sets) = get_sock_addr_and_app_settings();
    let make_svc = make_service_fn(move |_| {
        let s = sets.clone();
        async move { Ok::<_, Error>(service_fn(move |req| routes::service_route(req, s.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    let resp = call_service(
        Method::POST,
        addr.port(),
        routes::ROUTE_PATH_SIGHN_IN,
        Body::empty(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_signin_err() {
    let (addr, sets) = get_sock_addr_and_app_settings();
    let make_svc = make_service_fn(move |_| {
        let s = sets.clone();
        async move { Ok::<_, Error>(service_fn(move |req| routes::service_route(req, s.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    let resp = call_service(
        Method::POST,
        addr.port(),
        routes::ROUTE_PATH_SIGHN_IN,
        Body::empty(),
    )
    .await;
    assert_ne!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_ok() {
    let (addr, sets) = get_sock_addr_and_app_settings();
    let make_svc = make_service_fn(move |_| {
        let s = sets.clone();
        async move { Ok::<_, Error>(service_fn(move |req| routes::service_route(req, s.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    let resp = call_service(
        Method::POST,
        addr.port(),
        routes::ROUTE_PATH_SIGHN_UP,
        Body::empty(),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_err() {
    let (addr, sets) = get_sock_addr_and_app_settings();
    let make_svc = make_service_fn(move |_| {
        let s = sets.clone();
        async move { Ok::<_, Error>(service_fn(move |req| routes::service_route(req, s.clone()))) }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            eprintln!("server error: {}", err);
        }
    });

    let resp = call_service(
        Method::POST,
        addr.port(),
        routes::ROUTE_PATH_SIGHN_UP,
        Body::empty(),
    )
    .await;
    assert_ne!(resp.status(), StatusCode::UNAUTHORIZED);
}
