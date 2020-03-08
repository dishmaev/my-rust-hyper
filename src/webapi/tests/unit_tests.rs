use super::super::{connectors, handlers::*, errors};

#[tokio::test(threaded_scheduler)]
async fn test_signin_ok() {
    let reply = usr::signin().await.unwrap();
    assert_eq!(reply.error_code, errors::ErrorCode::ReplyOk);
}

#[tokio::test(threaded_scheduler)]
async fn test_signin_err() {
    let reply = usr::signin().await.unwrap();
    assert_ne!(reply.error_code, errors::ErrorCode::ReplyErrorDatabase);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_ok() {
    let reply = usr::signin().await.unwrap();
    assert_eq!(reply.error_code, errors::ErrorCode::ReplyOk);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_err() {
    let reply = usr::signin().await.unwrap();
    assert_ne!(reply.error_code, errors::ErrorCode::ReplyErrorDatabase);
}