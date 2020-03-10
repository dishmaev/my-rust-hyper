use super::super::{handlers::*, errors, connectors};

#[tokio::test(threaded_scheduler)]
async fn test_signin_ok() {
    let dc = connectors::DataConnector::new(None, None, None).await.unwrap();
    let reply = usr::signin(&dc).await.unwrap();
    assert_eq!(reply.error_code, errors::ErrorCode::ReplyOk);
}

#[tokio::test(threaded_scheduler)]
async fn test_signin_err() {
    let dc = connectors::DataConnector::new(None, None, None).await.unwrap();
    let reply = usr::signin(&dc).await.unwrap();
    assert_ne!(reply.error_code, errors::ErrorCode::ReplyErrorDatabase);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_ok() {
    let dc = connectors::DataConnector::new(None, None, None).await.unwrap();
    let reply = usr::signin(&dc).await.unwrap();
    assert_eq!(reply.error_code, errors::ErrorCode::ReplyOk);
}

#[tokio::test(threaded_scheduler)]
async fn test_signup_err() {
    let dc = connectors::DataConnector::new(None, None, None).await.unwrap();
    let reply = usr::signin(&dc).await.unwrap();
    assert_ne!(reply.error_code, errors::ErrorCode::ReplyErrorDatabase);
}

#[tokio::test(threaded_scheduler)]
async fn test_car_get_ok() {
    let dc = connectors::DataConnector::new(None, None, None).await.unwrap();
    let reply = car::get(&dc, None).await.unwrap();
    assert_eq!(reply.len(), 1);
}