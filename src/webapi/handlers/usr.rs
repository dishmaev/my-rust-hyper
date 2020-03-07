use super::super::{connectors, collections::*, errors, models};

pub async fn get(
    dc: &connectors::DataConnector,
    ids: Option<Vec<i32>>,
) -> connectors::Result<Vec<usr::Usr>> {
    Ok(dc.usr.get(ids).await?)
}
