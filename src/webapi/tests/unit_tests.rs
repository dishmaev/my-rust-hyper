use super::super::handlers;
use hyper::StatusCode;

#[tokio::test(threaded_scheduler)]
async fn test_signin_ok() {
    let resp = handlers::signin().await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_signin_err() {
    let resp = handlers::signin().await;
    assert_ne!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_ok() {
    let resp = handlers::signin().await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_err() {
    let resp = handlers::signin().await;
    assert_ne!(resp.status(), StatusCode::BAD_REQUEST);
}
