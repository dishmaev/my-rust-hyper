use super::super::{connectors, collections::*, errors, models};

pub async fn get(
    dc: &connectors::DataConnector,
    ids: Option<Vec<i32>>,
) -> connectors::Result<Vec<usr::Usr>> {
    Ok(dc.usr.get(ids).await?)
}

pub async fn signin() -> connectors::Result<models::Reply> {
    Ok(models::Reply {
        error_code: errors::ErrorCode::ReplyOk,
        error_name: None,
    })
}

pub async fn signup() -> connectors::Result<models::Reply> {
    Ok(models::Reply {
        error_code: errors::ErrorCode::ReplyOk,
        error_name: None,
    })
}