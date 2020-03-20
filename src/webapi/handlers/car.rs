use super::super::{connectors, executors, commands, entities, errors};
use super::models;

pub async fn get(
    dc: &connectors::DataConnector,
    ids: Option<Vec<i32>>,
) -> connectors::Result<Vec<entities::car::Car>> {
    Ok(dc.car.get(ids).await?)
}

pub async fn add(
    dc: &connectors::DataConnector,
    items: Vec<entities::car::Car>,
) -> connectors::Result<models::AddIntIdsReply> {
    let (result, ids) = dc.car.add(items).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_add_int_ids_reply!(ids.unwrap()))
    } else {
        Ok(get_error_add_int_ids_reply!(&result, dc.error))
    }
}

pub async fn update(
    dc: &connectors::DataConnector,
    ce: &executors::CommandExecutor,
    items: Vec<entities::car::Car>,
) -> connectors::Result<models::Reply> {
    let c: models::Reply = ce.call(None::<commands::car::MoveCar>).await?;
    debug!("{}", c.error_code.as_isize());
    let result: errors::ErrorCode = dc.car.update(items).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply!())
    } else {
        Ok(get_error_reply!(&result, dc.error))
    }
}

pub async fn remove(
    dc: &connectors::DataConnector,
    ids: Vec<i32>,
) -> connectors::Result<models::Reply> {
    let result: errors::ErrorCode = dc.car.remove(ids).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply!())
    } else {
        Ok(get_error_reply!(&result, dc.error))
    }
}
