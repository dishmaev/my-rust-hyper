#[macro_use]
extern crate log;
#[macro_use]
extern crate strum_macros;

extern crate chrono;

mod webapi;

use dotenv::dotenv;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use webapi::{access, connectors, executors, publishers, router, routes, settings, workers};

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

    #[cfg(feature = "postgres")]
    const DB_PG: &str = "pg";
    #[cfg(feature = "mysql")]
    const DB_MYSQL: &str = "mysql";

    let log_setting_file: String =
        env::var(ENV_LOG_SETTINGS).unwrap_or(String::from(DEFAULT_LOG_SETTINGS));
    log4rs::init_file(log_setting_file, Default::default()).unwrap();

    info!("initializing");

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

    #[cfg(feature = "postgres")]
    let db = app_settings.database.get(DB_PG).unwrap();
    #[cfg(feature = "mysql")]
    let db = app_settings.database.get(DB_MYSQL).unwrap();

    let data_connector = connectors::DataConnector::new(app_settings.error, db)
        .await
        .expect("error while data connector initialize");
    let access_checker = access::AccessChecker::from_data_connector(
        &data_connector,
        &app_settings.access.authentication,
    )
    .await
    .expect("error while access checker initialize");

    let data_connector_arc = Arc::new(data_connector);
    let access_checker_arc = Arc::new(access_checker);

    let router = router::Router::new(
        data_connector_arc.clone(),
        access_checker_arc.clone(),
        app_settings.router,
        app_settings.path,
        app_settings.service,
        &host,
    )
    .await
    .expect("error while remote router initialize");

    let router_arc = Arc::new(router);

    let (command_executor_control_sender, command_executor_control_receiver) =
        mpsc::channel::<workers::SignalCode>(5);
    let (event_publisher_control_sender, event_publisher_control_receiver) =
        mpsc::channel::<workers::SignalCode>(5);

    let control_senders = vec![
        event_publisher_control_sender.clone(),
        command_executor_control_sender.clone(),
    ];

    let command_executor = executors::CommandExecutor::new(
        access_checker_arc.clone(),
        router_arc.clone(),
        command_executor_control_sender.clone(),
    )
    .await
    .expect("error while command executor initialize");
    let event_publisher = publishers::EventPublisher::new(
        access_checker_arc.clone(),
        router_arc.clone(),
        event_publisher_control_sender.clone(),
    )
    .await
    .expect("error while event publisher initialize");

    let command_executor_arc = Arc::new(command_executor);
    let event_publisher_arc = Arc::new(event_publisher);

    let local_rt_arc = router_arc.clone();

    info!("starting up");
    debug!("start hyper server");

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

    let graceful = server
        .with_graceful_shutdown(async { shutdown_signal(cancel_flag, control_senders).await });

    let res = futures::join!(
        graceful,
        tokio::spawn(async move {
            if let Err(e) = workers::event_publisher_worker(
                event_publisher_cancel_flag,
                event_publisher_control_receiver,
            )
            .await
            {
                error!("event publisher: {}", e);
                return "error";
            }
            "ok"
        }),
        tokio::spawn(async move {
            if let Err(e) = workers::command_executor_worker(
                command_executer_cancel_flag,
                command_executor_control_receiver,
            )
            .await
            {
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
            .shutdown()
            .await
            .expect("error while router shutdown");
    }
    info!("shutdown");
}

async fn shutdown_signal(
    cancel_flag: Arc<AtomicBool>,
    control_senders: Vec<mpsc::Sender<workers::SignalCode>>,
) {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    cancel_flag.store(true, Ordering::SeqCst);
    for mut s in control_senders {
        let i = &mut s;
        i.send(workers::SignalCode::Exit).await.unwrap();
    }
}
