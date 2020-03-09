use super::super::{connectors, entities::usr, errors};
use super::models;

pub async fn get(
    dc: &connectors::DataConnector,
    ids: Option<Vec<i32>>,
) -> connectors::Result<Vec<usr::Usr>> {
    Ok(dc.usr.get(ids).await?)
}

pub async fn signin(dc: &connectors::DataConnector) -> connectors::Result<models::Reply> {
    Ok(get_ok_reply!())
}

pub async fn signup(dc: &connectors::DataConnector) -> connectors::Result<models::Reply> {
    Ok(get_ok_reply!())
}
