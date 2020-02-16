use super::super::handlers;
use futures_await_test::async_test;
use hyper::StatusCode;

#[async_test]
async fn test_signin_ok() {
    let resp = handlers::signin().await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[async_test]
async fn test_signin_err() {
    let resp = handlers::signin().await;
    assert_ne!(resp.status(), StatusCode::BAD_REQUEST);
}

#[async_test]
async fn test_signup_ok() {
    let resp = handlers::signin().await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[async_test]
async fn test_signup_err() {
    let resp = handlers::signin().await;
    assert_ne!(resp.status(), StatusCode::BAD_REQUEST);
}
