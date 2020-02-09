use hyper::{header, Body, Response, StatusCode};
use serde::{ser, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Event {
    id: Option<i32>,
    //    timestamp: f64,
    //    kind: String,
    //    tags: Vec<String>,
}

pub fn resp_with_code(status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .unwrap()
}

fn resp<T>(res: &T) -> Response<Body>
where
    T: ser::Serialize,
{
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&res).unwrap()))
        .unwrap()
}

pub async fn index() -> Response<Body> {
    resp_with_code(StatusCode::OK)
}

pub async fn index2() -> Response<Body> {
    let res = Event { id: Some(100) };
    resp(&res)
}

pub async fn index3(req: Event) -> Response<Body> {
    resp(&req)
}

pub async fn index4(req: Event) -> Response<Body> {
    if req.id != Some(0) {
        resp(&req)
    } else {
        resp_with_code(StatusCode::BAD_REQUEST)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use futures_await_test::async_test;

    #[async_test]
    async fn test_index1_ok() {
        let resp = index().await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[async_test]
    async fn test_index1_err() {
        let resp = index().await;
        assert_ne!(resp.status(), StatusCode::BAD_REQUEST);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use futures_await_test::async_test;

    #[async_test]
    async fn test_index3_ok() {
        
    }
}
