use super::{connectors, handlers, errors};
use serde::{de, ser};

pub struct CommandExecutor{
    http_command_producer: HttpCommandProducer
}

impl CommandExecutor{
    pub async fn new() -> connectors::Result<CommandExecutor> {
        Ok(CommandExecutor{ http_command_producer: HttpCommandProducer{}})
    }

    pub async fn call<T, R>(&self, object_type: String, request: Option<T>) -> connectors::Result<R>
        where T: ser::Serialize, R: for <'de> de::Deserialize<'de>,
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
        Ok(HttpCommandProducer{})
    }
}