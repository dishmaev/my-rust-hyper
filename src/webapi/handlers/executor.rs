use super::super::{connectors, errors, events, replies, router};

pub async fn on_async_command_state_change(
    dc: &connectors::DataConnector,
    rt: &router::Router,
    _items: Vec<events::executor::OnAsyncCommandStateChange>,
) -> connectors::Result<replies::common::StandardReply> {
    //todo: add state history for sended command
    Ok(get_ok_reply!())
}

pub async fn add_sended_async_command() -> connectors::Result<replies::common::StandardReply> {
    Ok(get_ok_reply!())
}

pub async fn remove_sended_async_command() -> connectors::Result<replies::common::StandardReply> {
    Ok(get_ok_reply!())
}

pub async fn add_received_async_command() -> connectors::Result<replies::common::StandardReply> {
    Ok(get_ok_reply!())
}

pub async fn remove_received_async_command() -> connectors::Result<replies::common::StandardReply> {
    Ok(get_ok_reply!())
}
