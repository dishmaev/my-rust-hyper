use super::{access, connectors, entities, errors, providers, router, traits, workers};
use hyper::Body;
use bytes::Buf;
use serde::{de, ser};
use std::collections::HashMap;
//use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, PartialEq, Copy, Clone, ToString)]
pub enum ExecMode {
    Any,   //default if ommit in call, priority for sync
    Sync,  //only sync
    Async, //only async
}

pub struct CommandExecutor {
    dc: Arc<connectors::DataConnector>,
    ac: Arc<access::AccessChecker>,
    rt: Arc<router::Router>,
    hp: providers::HttpProvider,
    _cs: mpsc::Sender<workers::SignalCode>,
}

impl CommandExecutor {
    pub async fn new(
        dc: Arc<connectors::DataConnector>,
        ac: Arc<access::AccessChecker>,
        rt: Arc<router::Router>,
        cs: mpsc::Sender<workers::SignalCode>,
    ) -> connectors::Result<CommandExecutor> {
        Ok(CommandExecutor {
            dc: dc,
            ac: ac,
            rt: rt,
            hp: providers::HttpProvider::new().await?,
            _cs: cs,
        })
    }

    pub async fn send_signal(&self, signal_code: workers::SignalCode) -> connectors::Result<()> {
        let mut s = self._cs.clone();
        match s.send(signal_code).await {
            Ok(_) => Ok({}),
            Err(e) => {
                error!("command executor: {}", e);
                return Err(errors::SignalSendError.into());
            }
        }
    }

    pub async fn change_received_async_command_state(
        &self,
        state: String,
        id: &str,
    ) -> connectors::Result<entities::executor::AsyncCommandState> {
        let c = self
            .dc
            .received_async_command
            .get(Some(vec![id.to_string()]))
            .await?;
        if c.len() == 1 {
            //todo: call change_state collection method
            Ok(entities::executor::AsyncCommandState {
                id: c[0].id.clone(),
                state: c[0].state.clone(),
                state_changed_at: c[0].state_changed_at,
            })
        } else {
            Err(errors::AsyncCommandNotFoundError.into())
        }
    }

    pub async fn get_received_async_command_state(
        &self,
        id: &str,
    ) -> connectors::Result<entities::executor::AsyncCommandState> {
        let c = self
            .dc
            .received_async_command
            .get(Some(vec![id.to_string()]))
            .await?;
        if c.len() == 1 {
            Ok(entities::executor::AsyncCommandState {
                id: c[0].id.clone(),
                state: c[0].state.clone(),
                state_changed_at: c[0].state_changed_at,
            })
        } else {
            Err(errors::AsyncCommandNotFoundError.into())
        }
    }

    pub async fn get_sended_async_command_state(
        &self,
        id: &str,
    ) -> connectors::Result<entities::executor::AsyncCommandState> {
        let sac = self
            .dc
            .sended_async_command
            .get(Some(vec![id.to_string()]))
            .await?;
        if sac.len() == 1 {
            let command = self.rt.get_command(&sac[0].object_type)?;
            let cid = Uuid::new_v4().to_hyphenated().to_string();
            if command
                .path
                .contains_key(&providers::Proto::http.to_string())
            {
                let mut prop = HashMap::<&str, &str>::new();
                prop.insert("correlation_id", &cid);
                prop.insert("async_command_id", &id);
                let token = self
                    .ac
                    .get_client_basic_authorization_token(command.service_name.as_ref().unwrap())?;
                let sp = self
                    .rt
                    .get_service_path(
                        &command.service_name.as_ref().unwrap(),
                        providers::Proto::http,
                    )?
                    .state;
                let response = self
                    .hp
                    .execute(
                        &self
                            .rt
                            .get_service_path(
                                &command.service_name.as_ref().unwrap(),
                                providers::Proto::http,
                            )?
                            .state,
                        prop,
                        token,
                        Body::empty(),
                    )
                    .await?;
                let reader = hyper::body::aggregate(response).await?.reader();
                let reply: Option<entities::executor::AsyncCommandState> =
                    serde_json::from_reader(reader).unwrap_or(None);
                if reply.is_some() {
                    Ok(reply.unwrap())
                } else {
                    Err(errors::BadReplyCommandError.into())
                }
            } else {
                Err(errors::UnsupportedProtoError.into())
            }
        } else {
            Err(errors::AsyncCommandNotFoundError.into())
        }
    }

    pub async fn call<T, R>(&self, request: T) -> connectors::Result<R>
    where
        T: ser::Serialize,
        T: traits::ObjectType,
        R: for<'de> de::Deserialize<'de>,
        R: traits::ObjectType,
    {
        let command = self.rt.get_command(T::get_type_name())?;
        if command
            .path
            .contains_key(&providers::Proto::http.to_string())
        {
            let cid = Uuid::new_v4().to_hyphenated().to_string();
            let mut prop = HashMap::<&str, &str>::new();
            prop.insert("correlation_id", &cid);
            prop.insert("object_type", T::get_type_name());
            let token = self
                .ac
                .get_client_basic_authorization_token(command.service_name.as_ref().unwrap())?;
            let response = self
                .hp
                .execute(
                    &command
                        .path
                        .get(&providers::Proto::http.to_string())
                        .unwrap(),
                    prop,
                    token,
                    Body::from(serde_json::to_string(&request).unwrap()),
                )
                .await?;
            let reader = hyper::body::aggregate(response).await?.reader();
            let reply: Option<R> = serde_json::from_reader(reader).unwrap_or(None);
            if reply.is_some() {
                Ok(reply.unwrap())
            } else {
                Err(errors::BadReplyCommandError.into())
            }
        } else {
            Err(errors::UnsupportedProtoError.into())
        }
    }
}
