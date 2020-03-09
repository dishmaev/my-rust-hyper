use super::super::{connectors, entities::car, errors};
use super::models;

pub async fn get(
    dc: &connectors::DataConnector,
    ids: Option<Vec<i32>>,
) -> connectors::Result<Vec<car::Car>> {
    Ok(dc.car.get(ids).await?)
}

pub async fn add(
    dc: &connectors::DataConnector,
    items: Vec<car::Car>,
) -> connectors::Result<models::AddReply> {
    let (result, ids) = dc.car.add(items).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_add_reply!(ids.unwrap()))
    } else {
        Ok(get_error_add_reply!(&result, dc.error))
    }
}

pub async fn update(
    dc: &connectors::DataConnector,
    items: Vec<car::Car>,
) -> connectors::Result<models::Reply> {
    let result: errors::ErrorCode = dc.car.update(items).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply!())
    } else {
        Ok(get_error_reply!(&result, dc.error))
    }
}

pub async fn delete(
    dc: &connectors::DataConnector,
    ids: Vec<i32>,
) -> connectors::Result<models::Reply> {
    let result: errors::ErrorCode = dc.car.delete(ids).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply!())
    } else {
        Ok(get_error_reply!(&result, dc.error))
    }
}
