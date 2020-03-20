use super::{connectors, traits};
use serde::{de, ser};

pub struct CommandExecutor {
    _http_command_producer: HttpCommandProducer,
}

impl CommandExecutor {
    pub async fn new() -> connectors::Result<CommandExecutor> {
        Ok(CommandExecutor {
            _http_command_producer: HttpCommandProducer::new().await?,
        })
    }

    pub async fn call<T, R>(&self, _request: Option<T>) -> connectors::Result<R>
    where
        T: ser::Serialize,
        T: traits::ObjectType,
        R: for<'de> de::Deserialize<'de>,
        R: traits::ObjectType,
    {
        let s = "{
            \"errorCode\": 0
        }";
        // let r: handlers::models::Reply = handlers::models::Reply{ error_code: errors::ErrorCode::ReplyOk, error_name: None}; //serde_json::from_str(s);
        let r: R = serde_json::from_str(s).unwrap();
        Ok(r)
    }
}

pub struct HttpCommandProducer;

impl HttpCommandProducer {
    pub async fn new() -> connectors::Result<HttpCommandProducer> {
        Ok(HttpCommandProducer {})
    }
}
