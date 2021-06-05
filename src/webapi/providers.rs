use super::{connectors, errors, entities};
use hyper::{Body, Client, Request, Method, StatusCode};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
#[cfg(feature = "mysql")]
use sqlx::MySqlPool;

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone, ToString, JsonSchema)]
pub enum Proto {
    Http,
    Mq
}

pub struct HttpProvider;

impl HttpProvider {
    pub async fn new() -> connectors::Result<HttpProvider> {
        Ok(HttpProvider {})
    }

    pub async fn execute(
        &self,
        to: &str,
        prop: HashMap<&str, &str>,
        bat: String,
        body: Body,
    ) -> connectors::Result<Body> {
        let mut uri = String::from("");
        for item in prop.iter(){
            if uri.len() != 0 {
                uri.push_str(&format!("&{}={}", item.0, item.1));
            }
            else
            {
                uri.push_str(&format!("?{}={}", item.0, item.1));
            }
        }
        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", to, uri))
            .header("Authorization", bat)
            .body(body)
            .expect("request builder");
        let client = Client::new();
        let resp = client.request(req).await?;
        let (parts, body) = resp.into_parts();
        if parts.status == StatusCode::OK {
            Ok(body)
        }
        else{
            Err(errors::ProtoProviderError.into())
        }
    }
}

pub struct MqProvider;

impl MqProvider {
    pub async fn new() -> connectors::Result<MqProvider> {
        Ok(MqProvider {})
    }

    pub async fn execute(
        &self,
        _to: &str,
        _prop: HashMap<&str, &str>,
        _body: Body,
    ) -> connectors::Result<Body> {
        //todo:
        Ok(Body::empty())
    }
}

#[cfg(not(test))]
pub struct SqlDbProvider {
    #[cfg(feature = "postgres")]
    pub pool: Arc<PgPool>,
    #[cfg(feature = "mysql")]
    pub pool: Arc<MySqlPool>,
}

#[cfg(not(test))]
impl SqlDbProvider {
    pub async fn new(connection_string: &str) -> connectors::Result<SqlDbProvider> {
        debug!("connection string {}", connection_string);
        #[cfg(feature = "postgres")]
        let pool = PgPool::connect(&connection_string).await?;
        #[cfg(feature = "mysql")]
        let pool = MySqlPool::connect(&connection_string).await?;
        Ok(SqlDbProvider {
            pool: Arc::new(pool),
        })
    }

    pub async fn get_errors(&self) -> connectors::Result<Vec<entities::error::Error>> {
        Ok(
            vec![entities::error::Error{
                error_code: errors::ErrorCode::DatabaseError.to_string(),
                error_name: "Database error".to_string()
            },
            entities::error::Error{
                error_code: errors::ErrorCode::NotFoundError.to_string(),
                error_name: "Some items with specified id is not found".to_string()
            }]
        )
    }
}
