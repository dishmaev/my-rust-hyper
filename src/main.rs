#[macro_use]
extern crate log;

mod webapi;

use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use webapi::{access, connectors, executors, publishers, router, routes, settings};

#[tokio::main]
async fn main() {
    dotenv().ok();

    const DEFAULT_APP_SETTINGS: &str = "appsettings.json";
    const ENV_APP_SETTINGS: &str = "MY_APP_SETTINGS";

    const DEFAULT_LOG_SETTINGS: &str = "log4rs.yml";
    const ENV_LOG_SETTINGS: &str = "MY_LOG_SETTINGS";

    const DEFAULT_HOST: &str = "127.0.0.1";
    const ENV_HOST: &str = "MY_BIN_HOST";

    const DEFAULT_PORT: u16 = 3456;
    const ENV_PORT: &str = "PORT";

    let log_setting_file: String =
        env::var(ENV_LOG_SETTINGS).unwrap_or(String::from(DEFAULT_LOG_SETTINGS));
    log4rs::init_file(log_setting_file, Default::default()).unwrap();

    info!("initializing...");

    let host_name: Option<String> = {
        match env::var(ENV_HOST).is_ok() {
            true => Some(env::var(ENV_HOST).unwrap()),
            _ => None,
        }
    };

    let port_name: Option<u16> = {
        match env::var(ENV_PORT).is_ok() {
            true => Some(env::var(ENV_PORT).unwrap().parse::<u16>().unwrap()),
            _ => None,
        }
    };

    let host = format!(
        "{}:{}",
        host_name.unwrap_or(String::from(DEFAULT_HOST)),
        port_name.unwrap_or(DEFAULT_PORT)
    );

    let addr = host.parse::<SocketAddr>().unwrap();

    let app_setting_file: String =
        env::var(ENV_APP_SETTINGS).unwrap_or(String::from(DEFAULT_APP_SETTINGS));

    let app_settings: settings::AppSettings =
        serde_json::from_str(&fs::read_to_string(app_setting_file).unwrap()).unwrap();

    let data_connector = connectors::DataConnector::new(
        app_settings._error,
        Some(app_settings._pg_db),
        Some(app_settings._my_sql_db),
    )
    .await
    .expect("error while data connector initialize");
    let access_checker = access::AccessChecker::from_data_connector(&data_connector)
        .await
        .expect("error while access checker initialize");

    let router = if app_settings._router.local.is_some() {
        router::Router::from_local(
            &data_connector,
            app_settings._router.local.unwrap(),
            app_settings._service,
            &host,
        )
        .await
        .expect("error while router initialize")
    } else {
        router::Router::from_remote(
            app_settings._router.http_from,
            app_settings._router.mq_from,
            app_settings._service,
            &host,
        )
        .await
        .expect("error while router initialize")
    };
    let command_executor = executors::CommandExecutor::new()
        .await
        .expect("error while command executor initialize");

    let event_publisher = publishers::EventPublisher::new()
        .await
        .expect("error while event publisher initialize");

    let data_connector_arc = Arc::new(data_connector);
    let access_checker_arc = Arc::new(access_checker);
    let command_executor_arc = Arc::new(command_executor);
    let event_publisher_arc = Arc::new(event_publisher);
    let router_arc = Arc::new(router);

    let local_rt_arc = router_arc.clone();
    let local_dc_arc = data_connector_arc.clone();

    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        let ce = command_executor_arc.clone();
        let ep = event_publisher_arc.clone();
        let rt = router_arc.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                routes::service::service_route(
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
    let server = Server::bind(&addr).serve(make_svc);
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let event_publisher_cancel_flag = cancel_flag.clone();
    let command_executer_cancel_flag = cancel_flag.clone();
    let graceful = server.with_graceful_shutdown(async { shutdown_signal(cancel_flag).await });

    info!("starting up");
    debug!("start hyper server");
    let res = futures::join!(
        graceful,
        tokio::spawn(async move {
            debug!("start event publisher");
            if let Err(e) = event_publisher_task(event_publisher_cancel_flag).await {
                error!("event publisher: {}", e);
                return "error";
            }
            "ok"
        }),
        tokio::spawn(async move {
            debug!("start command executor");
            if let Err(e) = command_executor_task(command_executer_cancel_flag).await {
                error!("command executor: {}", e);
                return "error";
            }
            "ok"
        })
    );
    debug!("stop command executor with result: {}", (res.2.unwrap()));
    debug!("stop event publisher with result: {}", (res.1.unwrap()));
    if let Err(e) = res.0 {
        error!("hyper server: {}", e);
        debug!("stop hyper server with result: error");
    } else {
        debug!("stop hyper server with result: ok");
        local_rt_arc
            .shutdown(local_dc_arc)
            .await
            .expect("error while router shutdown");
    }
    info!("shutdown");
}

async fn shutdown_signal(cancel_flag: Arc<AtomicBool>) {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    cancel_flag.store(true, Ordering::SeqCst);
}

async fn event_publisher_task(cancel_flag: Arc<AtomicBool>) -> connectors::Result<()> {
    while !cancel_flag.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_secs(2));
        // debug!("event publisher in action");
    }
    debug!("event publisher exit");
    Ok({})
}

async fn command_executor_task(cancel_flag: Arc<AtomicBool>) -> connectors::Result<()> {
    while !cancel_flag.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_secs(3));
        // debug!("command executor in action");
    }
    debug!("command executor exit");
    Ok({})
}
