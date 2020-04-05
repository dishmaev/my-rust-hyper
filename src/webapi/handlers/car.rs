use super::super::{commands, connectors, errors, executors, replies};

pub async fn get(
    dc: &connectors::DataConnector,
    cmd: commands::car::GetCar,
) -> connectors::Result<replies::car::GetCarReply> {
    match dc.car.get(cmd.ids).await {
        Ok(r) => {
            Ok(replies::car::GetCarReply{
                error_code: errors::ErrorCode::ReplyOk,
                error_name: None,
                url: None,
                items: Some(r)
            })
        },
        Err(e) => {
            error!("get_car handler get car collection: {}", e);
            let ec = errors::ErrorCode::DatabaseError;
            Ok(replies::car::GetCarReply{
                error_code: ec.clone(),
                error_name: Some(dc.error.get(&ec.to_string()).unwrap().clone()),
                url: None,
                items: None
            })
        }
    }
}

pub async fn add(
    dc: &connectors::DataConnector,
    cmd: commands::car::AddCar,
) -> connectors::Result<replies::common::AddIntIdsReply> {
    let (result, ids) = dc.car.add(cmd.items).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_add_int_ids_reply!(ids.unwrap()))
    } else {
        Ok(get_error_add_int_ids_reply!(&result, dc.error))
    }
}

pub async fn modify(
    dc: &connectors::DataConnector,
    ce: &executors::CommandExecutor,
    cmd: commands::car::ModifyCar,
) -> connectors::Result<replies::common::StandardReply> {
    let _c: replies::common::StandardReply = ce
        .call(commands::car::ReserveCar { services: vec![1] })
        .await?;
    let result: errors::ErrorCode = dc.car.modify(cmd.items).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply!())
    } else {
        Ok(get_error_reply!(&result, dc.error))
    }
}

pub async fn remove(
    dc: &connectors::DataConnector,
    cmd: commands::car::RemoveCar,
) -> connectors::Result<replies::common::StandardReply> {
    let result: errors::ErrorCode = dc.car.remove(cmd.ids).await?;
    if result == errors::ErrorCode::ReplyOk {
        Ok(get_ok_reply!())
    } else {
        Ok(get_error_reply!(&result, dc.error))
    }
}

pub async fn reserve(
    _dc: &connectors::DataConnector,
    _cmd: commands::car::ReserveCar,
) -> connectors::Result<replies::common::StandardReply> {
    Ok(get_ok_reply!())
}
