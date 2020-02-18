mod webapi;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use webapi::{models, routes};

#[tokio::main]
async fn main() {
    const ENV_HOST: &str = "MY_BIN_HOST";
    const ENV_PORT: &str = "PORT";
    const DEFAULT_HOST: &str = "127.0.0.1";
    const DEFAULT_PORT: u16 = 3456;
    const APP_SETTINGS_FILE: &str = "appsettings.json";

    let host: Option<String> = {
        match env::var(ENV_HOST).is_ok() {
            true => Some(env::var(ENV_HOST).unwrap()),
            _ => None,
        }
    };

    let port: Option<u16> = {
        match env::var(ENV_HOST).is_ok() {
            true => Some(env::var(ENV_PORT).unwrap().parse::<u16>().unwrap()),
            _ => None,
        }
    };

    let addr = format!(
        "{}:{}",
        host.unwrap_or(String::from(DEFAULT_HOST)),
        port.unwrap_or(DEFAULT_PORT)
    )
    .parse::<SocketAddr>()
    .unwrap();

    let config_file: models::AppSettings =
        serde_json::from_str(&fs::read_to_string(APP_SETTINGS_FILE).unwrap()).unwrap();
    let access_checker = routes::AccessChecker {
        user_password: config_file.authentication,
    };
    access_checker.initialize();
    let access_checker_arc = Arc::new(access_checker);

    let make_svc = make_service_fn(move |_| {
        let ac = access_checker_arc.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                routes::service_route(req, ac.clone())
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
