mod handlers;
use bytes::buf::BufExt;
use handlers::{index, index2, index3, index4, resp_with_code};
use hyper::service::{make_service_fn, service_fn};
use hyper::{error::Result, Body, Error, Method, Request, Response, Server, StatusCode};
use std::{env, net::SocketAddr};

async fn router(req: Request<Body>) -> Result<Response<Body>> {
    const PATH_ROOT: &str = "/";
    const PATH_2: &str = "/2";
    const PATH_3: &str = "/3";
    const PATH_4: &str = "/4";

    let (parts, body) = req.into_parts();
    let reader = hyper::body::aggregate(body).await?.reader();

    let resp = {
        match (parts.method, parts.uri.path()) {
            (Method::POST, PATH_ROOT) => index().await,
            (Method::POST, PATH_2) => index2().await,
            (Method::POST, PATH_3) => index3(serde_json::from_reader(reader).unwrap()).await,
            (Method::POST, PATH_4) => index4(serde_json::from_reader(reader).unwrap()).await,
            _ => resp_with_code(StatusCode::NOT_FOUND),
        }
    };
    Ok(resp)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    const DEFAULT_HOST: &str = "127.0.0.1";
    const DEFAULT_PORT: &str = "3456";
    const ENV_HOST: &str = "MY_BIN_HOST";
    const ENV_PORT: &str = "PORT";

    let host = env::var(ENV_HOST).unwrap_or(String::from(DEFAULT_HOST));
    let port = env::var(ENV_PORT)
        .unwrap_or(String::from(DEFAULT_PORT))
        .parse::<usize>()
        .unwrap();

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Error>(service_fn(router)) });

    let server =
        Server::bind(&format!("{}:{}", host, port).parse::<SocketAddr>().unwrap()).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
