use super::super::{connectors, collections::*, errors, models};

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
    Ok(dc.car.add(items).await?)
}

pub async fn update(
    dc: &connectors::DataConnector,
    items: Vec<car::Car>,
) -> connectors::Result<models::Reply> {
    Ok(dc.car.update(items).await?)
}

pub async fn delete(
    dc: &connectors::DataConnector,
    ids: Vec<i32>,
) -> connectors::Result<models::Reply> {
    Ok(dc.car.delete(ids).await?)
}
