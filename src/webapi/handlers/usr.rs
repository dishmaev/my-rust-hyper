use super::super::{connectors, entities::usr, errors, replies};

pub async fn get(
    dc: &connectors::DataConnector,
    ids: Option<Vec<i32>>,
) -> connectors::Result<Vec<usr::Usr>> {
    Ok(dc.usr.get(ids).await?)
}

pub async fn signin(_dc: &connectors::DataConnector) -> connectors::Result<replies::common::StandardReply> {
    Ok(get_ok_reply!())
}

pub async fn signup(_dc: &connectors::DataConnector) -> connectors::Result<replies::common::StandardReply> {
    Ok(get_ok_reply!())
}
