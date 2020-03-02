mod webapi;

use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use webapi::{collections, models, routes};

#[tokio::main]
async fn main() {
    dotenv().ok();

    const DEFAULT_APP_SETTINGS: &str = "appsettings.json";
    const ENV_APP_SETTINGS: &str = "MY_APP_SETTINGS";

    const DEFAULT_HOST: &str = "127.0.0.1";
    const ENV_HOST: &str = "MY_BIN_HOST";

    const DEFAULT_PORT: u16 = 3456;
    const ENV_PORT: &str = "PORT";

    let file: String = env::var(ENV_APP_SETTINGS).unwrap_or(String::from(DEFAULT_APP_SETTINGS));

    let host: Option<String> = {
        match env::var(ENV_HOST).is_ok() {
            true => Some(env::var(ENV_HOST).unwrap()),
            _ => None,
        }
    };

    let port: Option<u16> = {
        match env::var(ENV_PORT).is_ok() {
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

    let app_settings: models::AppSettings =
        serde_json::from_str(&fs::read_to_string(file).unwrap()).unwrap();

    let data_connector = collections::DataConnector::new(app_settings.pgDb)
        .await
        .expect("error while initialize data connector");
    let access_checker = routes::AccessChecker::from_data_connector(&data_connector)
        .await
        .expect("error while initialize access checker");

    let data_connector_arc = Arc::new(data_connector);
    let access_checker_arc = Arc::new(access_checker);

    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                routes::service_route(req, dc.clone(), ac.clone())
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
