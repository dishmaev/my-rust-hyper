#[macro_use]
extern crate log;
#[macro_use]
extern crate strum_macros;

extern crate chrono;

mod webapi;

use dotenv::dotenv;

#[cfg(feature = "amqp")]
use dove::container::*;
#[cfg(feature = "amqp")]
use dove::url;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Server};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::Duration;
use webapi::{access, connectors, executors, publishers, router, routes, settings, workers};

pub fn sync_command_handler(
    _body: Body,
    _param: HashMap<String, String>,
) -> (Body, HashMap<String, String>) {
    info!("command handler");
    (Body::empty(), HashMap::<String, String>::new())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    const DEFAULT_HOST: &str = "127.0.0.1";
    const ENV_HOST: &str = "MY_BIN_HOST";

    const DEFAULT_PORT: u16 = 3456;
    const ENV_PORT: &str = "PORT";

    const DEFAULT_APP_SETTINGS: &str = "appsettings.dev.json";
    const ENV_APP_SETTINGS: &str = "MY_APP_SETTINGS";

    const DEFAULT_LOG_SETTINGS: &str = "log4rs.yml";
    const ENV_LOG_SETTINGS: &str = "MY_LOG_SETTINGS";

    const ENV_DATABASE_URL: &str = "DATABASE_URL";
    const ENV_MQ_BROKER: &str = "MQ_BROKER";

    #[cfg(feature = "postgres")]
    const DB_PG: &str = "pg";
    #[cfg(feature = "mysql")]
    const DB_MYSQL: &str = "mysql";

    #[cfg(feature = "amqp")]
    const MQ_AMQP: &str = "ampq";
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
    let db_connection_string =
        env::var(ENV_DATABASE_URL).unwrap_or(app_settings.database.get(DB_PG).unwrap().to_string());
    #[cfg(feature = "mysql")]
    let db_connection_string = env::var(ENV_DATABASE_URL)
        .unwrap_or(app_settings.database.get(DB_MYSQL).unwrap().to_string());

    let data_connector = connectors::DataConnector::new(app_settings.error, &db_connection_string)
        .await
        .expect("error while data connector initialize");
    let access_checker = access::AccessChecker::from_data_connector(
        &data_connector,
        &app_settings.access.authentication,
    )
    .await
    .expect("error while access checker initialize");

    #[cfg(feature = "amqp")]
    let mq_connection_string =
        env::var(ENV_MQ_BROKER).unwrap_or(app_settings.mq_broker.get(MQ_AMQP).unwrap().to_string());

    let url = url::Url::parse(&mq_connection_string).expect("error parsing url");
    let opts = ConnectionOptions {
        username: url.username.map(|s| s.to_string()),
        password: url.password.map(|s| s.to_string()),
        sasl_mechanism: url.username.map_or(Some(SaslMechanism::Anonymous), |_| {
            Some(SaslMechanism::Plain)
        }),
        idle_timeout: Some(Duration::from_secs(5)),
    };

    let container = Container::new()
        .expect("error while create mq container")
        .start();

    let broker = format!("{}:{}", url.hostname, url.port);

    let connection = container
        .connect(broker.clone(), opts)
        .await
        .expect("error while ceate mq connection");

    let _session = connection
        .new_session(None)
        .await
        .expect("error while create mq session");

    container.start();

    let data_connector_arc = Arc::new(data_connector);
    let access_checker_arc = Arc::new(access_checker);

    let router = router::Router::new(
        data_connector_arc.clone(),
        access_checker_arc.clone(),
        app_settings.router,
        app_settings.path,
        app_settings.service,
        &host,
        &broker,
    )
    .await
    .expect("error while remote router initialize");

    let router_arc = Arc::new(router);

    let (command_executor_control_sender, command_executor_control_receiver) =
        mpsc::channel::<workers::SignalCode>(5);
    let (event_publisher_control_sender, event_publisher_control_receiver) =
        mpsc::channel::<workers::SignalCode>(5);
    let (mq_sender_control_sender, mq_sender_control_receiver) =
        mpsc::channel::<workers::SignalCode>(5);
    let (mq_receiver_control_sender, mq_receiver_control_receiver) =
        mpsc::channel::<workers::SignalCode>(5);

    let control_senders = vec![
        command_executor_control_sender.clone(),
        event_publisher_control_sender.clone(),
        mq_sender_control_sender.clone(),
        mq_receiver_control_sender.clone(),
    ];

    let command_executor = executors::CommandExecutor::new(
        data_connector_arc.clone(),
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

    let mut hhm = HashMap::<&str, routes::service::Handler>::new();
    hhm.insert("s1", sync_command_handler);
    let handler_arc = Arc::new(hhm);

    let make_svc = make_service_fn(move |_| {
        let dc = data_connector_arc.clone();
        let ac = access_checker_arc.clone();
        let ce = command_executor_arc.clone();
        let ep = event_publisher_arc.clone();
        let rt = router_arc.clone();
        let hr = handler_arc.clone();

        async move {
            Ok::<_, Error>(service_fn(move |req| {
                routes::service::service_route(
                    req,
                    dc.clone(),
                    ac.clone(),
                    ce.clone(),
                    ep.clone(),
                    rt.clone(),
                    hr.clone(),
                )
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);

    info!("starting up");
    debug!("start hyper server {}", &addr);

    let cancel_flag = Arc::new(AtomicBool::new(false));
    let command_executer_cancel_flag = cancel_flag.clone();
    let event_publisher_cancel_flag = cancel_flag.clone();
    let mq_sender_cancel_flag = cancel_flag.clone();
    let mq_receiver_cancel_flag = cancel_flag.clone();

    let graceful = server
        .with_graceful_shutdown(async { shutdown_signal(cancel_flag, control_senders).await });

    let res = futures::join!(
        graceful,
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
        }),
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
            if let Err(e) =
                workers::mq_sender_worker(mq_sender_cancel_flag, mq_sender_control_receiver).await
            {
                error!("mq sender: {}", e);
                return "error";
            }
            "ok"
        }),
        tokio::spawn(async move {
            if let Err(e) =
                workers::mq_receiver_worker(mq_receiver_cancel_flag, mq_receiver_control_receiver)
                    .await
            {
                error!("mq receiver: {}", e);
                return "error";
            }
            "ok"
        }),
    );
    debug!("stop command executor with result: {}", (res.1.unwrap()));
    debug!("stop event publisher with result: {}", (res.2.unwrap()));
    debug!("stop mq sender with result: {}", (res.3.unwrap()));
    debug!("stop mq receiver with result: {}", (res.4.unwrap()));
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
    connection.close(None).unwrap();
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
