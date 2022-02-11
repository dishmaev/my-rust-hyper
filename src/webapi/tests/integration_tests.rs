/*
use super::super::{
    access, connectors, executors, publishers, router, routes::*, settings, workers,
};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Method, Request, Response, Server, StatusCode};
use rand::prelude::Rng;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::mpsc;

fn get_basic_authorization_token(user: &String, password: &String) -> String {
    format!(
        "Basic {}",
        base64::encode(&format!("{}:{}", user, password))
    )
}

async fn get_settings() -> (
    SocketAddr,
    Arc<connectors::DataConnector>,
    Arc<access::AccessChecker>,
    Arc<executors::CommandExecutor>,
    Arc<publishers::EventPublisher>,
    Arc<router::Router>,
) {
    let mut rng = rand::thread_rng();
    let app_settings: settings::AppSettings =
        serde_json::from_str(&fs::read_to_string("appsettings.test.json").unwrap()).unwrap();
    let data_connector = connectors::DataConnector::new(
        app_settings.error,
        Some(app_settings.pg_db),
        Some(app_settings.my_sql_db),
    )
    .await
    .expect("error while initialize data connector");
    let access_checker = access::AccessChecker::_from_app_settings(&app_settings.access)
        .await
        .expect("error while initialize access checker");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), rng.gen_range(15000, 25000));
    let host = format!("{}:{}", &addr.ip(), &addr.port());
    let router = router::Router::new_local(
        &data_connector,
        app_settings.router.local.unwrap(),
        app_settings.service,
        &host,
    )
    .await
    .expect("error while local router initialize");
    let access_checker_arc = Arc::new(access_checker);
    let router_arc = Arc::new(router);
    let (command_sender, _command_receiver) = mpsc::channel::<workers::SignalCode>(10);
    let command_executor = executors::CommandExecutor::new(access_checker_arc.clone(), router_arc.clone(), command_sender)
        .await
        .expect("error while initialize command executor");
    let (event_sender, _event_receiver) = mpsc::channel::<workers::SignalCode>(10);
    let event_publisher = publishers::EventPublisher::new(access_checker_arc.clone(), router_arc.clone(), event_sender)
        .await
        .expect("error while event publisher initialize");
    (
        addr,
        Arc::new(data_connector),
        access_checker_arc,
        Arc::new(command_executor),
        Arc::new(event_publisher),
        router_arc,
    )
}

async fn call_service(method: hyper::Method, port: u16, path: &str, body: Body) -> Response<Body> {
    let req = Request::builder()
        .method(method)
        .uri(format!("http://{}:{}{}", Ipv4Addr::LOCALHOST, port, path))
        .header(
            "Authorization",
            get_basic_authorization_token(&"test".to_string(), &"1234567890".to_string()),
        )
        .body(body)
        .expect("request builder");
    let client = Client::new();
    client.request(req).await.unwrap()
}

#[tokio::test(threaded_scheduler)]
async fn test_index_ok() {
    let (
        addr,
        data_connector_arc,
        access_checker_arc,
        command_executor_arc,
        event_publisher_arc,
        router_arc,
    ) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        let ce = command_executor_arc.clone();
        let ep = event_publisher_arc.clone();
        let rt = router_arc.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                service::service_route(
                    req,
                    dc.clone(),
                    ac.clone(),
                    ce.clone(),
                    ep.clone(),
                    rt.clone(),
                )
            }))
        }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            error!("server: {}", err);
        }
    });

    let resp = call_service(Method::GET, addr.port(), "/", Body::empty()).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_spec_json_ok() {
    let (
        addr,
        data_connector_arc,
        access_checker_arc,
        command_executor_arc,
        event_publisher_arc,
        router_arc,
    ) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        let ce = command_executor_arc.clone();
        let ep = event_publisher_arc.clone();
        let rt = router_arc.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                service::service_route(
                    req,
                    dc.clone(),
                    ac.clone(),
                    ce.clone(),
                    ep.clone(),
                    rt.clone(),
                )
            }))
        }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            error!("server: {}", err);
        }
    });

    let resp = call_service(Method::GET, addr.port(), "/openapi.json", Body::empty()).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_spec_yaml_ok() {
    let (
        addr,
        data_connector_arc,
        access_checker_arc,
        command_executor_arc,
        event_publisher_arc,
        router_arc,
    ) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        let ce = command_executor_arc.clone();
        let ep = event_publisher_arc.clone();
        let rt = router_arc.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                service::service_route(
                    req,
                    dc.clone(),
                    ac.clone(),
                    ce.clone(),
                    ep.clone(),
                    rt.clone(),
                )
            }))
        }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            error!("server: {}", err);
        }
    });

    let resp = call_service(Method::GET, addr.port(), "/openapi.yaml", Body::empty()).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_route_ok() {
    let (
        addr,
        data_connector_arc,
        access_checker_arc,
        command_executor_arc,
        event_publisher_arc,
        router_arc,
    ) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        let ce = command_executor_arc.clone();
        let ep = event_publisher_arc.clone();
        let rt = router_arc.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                service::service_route(
                    req,
                    dc.clone(),
                    ac.clone(),
                    ce.clone(),
                    ep.clone(),
                    rt.clone(),
                )
            }))
        }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            error!("server: {}", err);
        }
    });

    for route in path::ROUTE_WITH_EMPTY_BODY.iter() {
        let resp = call_service(Method::POST, addr.port(), route, Body::empty()).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

#[tokio::test(threaded_scheduler)]
async fn test_route_err() {
    let (
        addr,
        data_connector_arc,
        access_checker_arc,
        command_executor_arc,
        event_publisher_arc,
        router_arc,
    ) = get_settings().await;
    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        let ce = command_executor_arc.clone();
        let ep = event_publisher_arc.clone();
        let rt = router_arc.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                service::service_route(
                    req,
                    dc.clone(),
                    ac.clone(),
                    ce.clone(),
                    ep.clone(),
                    rt.clone(),
                )
            }))
        }
    });
    let app = Server::bind(&addr).serve(make_svc);

    tokio::spawn(async move {
        if let Err(err) = app.await {
            error!("server: {}", err);
        }
    });

    let resp = call_service(Method::POST, addr.port(), "/fake", Body::from("{}")).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

*/
