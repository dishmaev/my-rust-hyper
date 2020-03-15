use super::super::super::{connectors, entities::car, errors};

pub struct CarCollection {
    items: Vec<car::Car>,
}

impl CarCollection {
    pub fn new() -> CarCollection {
        let items = vec![car::Car {
            id: Some(1),
            car_name: "Test car".to_string(),
        }];
        CarCollection { items: items }
    }
    pub async fn get(&self, _ids: Option<Vec<i32>>) -> connectors::Result<Vec<car::Car>> {
        Ok(self.items.clone())
    }
    pub async fn add(
        &self,
        _items: Vec<car::Car>,
    ) -> connectors::Result<(errors::ErrorCode, Option<Vec<i32>>)> {
        Ok((errors::ErrorCode::ReplyOk, None))
    }
    pub async fn update(&self, _items: Vec<car::Car>) -> connectors::Result<errors::ErrorCode> {
        Ok(errors::ErrorCode::ReplyOk)
    }
    pub async fn remove(&self, _ids: Vec<i32>) -> connectors::Result<errors::ErrorCode> {
        Ok(errors::ErrorCode::ReplyOk)
    }
}
