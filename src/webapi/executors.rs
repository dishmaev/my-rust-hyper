use super::{access, connectors, errors, router, traits, workers};
use hyper::Client;
use serde::{de, ser};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct CommandExecutor {
    ac: Arc<access::AccessChecker>,
    rt: Arc<router::Router>,
    cs: mpsc::Sender<workers::SignalCode>,
    hcp: HttpCommandProducer,
    mcp: MqCommandProducer,
}

impl CommandExecutor {
    pub async fn new(
        ac: Arc<access::AccessChecker>,
        rt: Arc<router::Router>,
        cs: mpsc::Sender<workers::SignalCode>,
    ) -> connectors::Result<CommandExecutor> {
        Ok(CommandExecutor {
            ac: ac,
            rt: rt,
            cs: cs,
            hcp: HttpCommandProducer::new().await?,
            mcp: MqCommandProducer::new().await?,
        })
    }

    pub async fn send_signal(&self, signal_code: workers::SignalCode) -> connectors::Result<()> {
        let mut s = self.cs.clone();
        match s.send(signal_code).await {
            Ok(_) => Ok({}),
            Err(e) => {
                error!("command executor: {}", e);
                return Err(errors::SignalSendError.into());
            }
        }
    }

    pub async fn call<T, R>(&self, _request: Option<T>) -> connectors::Result<R>
    where
        T: ser::Serialize,
        T: traits::ObjectType,
        R: for<'de> de::Deserialize<'de>,
        R: traits::ObjectType,
    {
        let cr = self.rt.get_command(T::get_type_name())?;
        let correlation_id = Uuid::new_v4().to_hyphenated().to_string();
        debug!(
            "correlation id {} object_type {}",
            correlation_id,
            T::get_type_name()
        );
        let s = "{
            \"errorCode\": 0
        }";
        // let r: handlers::models::Reply = handlers::models::Reply{ error_code: errors::ErrorCode::ReplyOk, error_name: None}; //serde_json::from_str(s);
        let r: R = serde_json::from_str(s).unwrap();
        Ok(r)
    }
}

pub struct HttpCommandProducer {}

impl HttpCommandProducer {
    pub async fn new() -> connectors::Result<HttpCommandProducer> {
        Ok(HttpCommandProducer {})
    }
}

pub struct MqCommandProducer;

impl MqCommandProducer {
    pub async fn new() -> connectors::Result<MqCommandProducer> {
        Ok(MqCommandProducer {})
    }
}
