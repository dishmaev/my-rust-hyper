/*
use super::super::{handlers};

#[tokio::test(threaded_scheduler)]
async fn test_signin_ok() {
    let reply = handlers::signin().await;
    assert_eq!(reply.error_code, 0);
}

#[tokio::test(threaded_scheduler)]
async fn test_signin_err() {
    let reply = handlers::signin().await;
    assert_ne!(reply.error_code, -1);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_ok() {
    let reply = handlers::signin().await;
    assert_eq!(reply.error_code, 0);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_err() {
    let reply = handlers::signin().await;
    assert_ne!(reply.error_code, -1);
}
*/