use super::super::super::{connectors, entities::usr};

pub struct UsrCollection {
    items: Vec<usr::Usr>,
}

impl UsrCollection {
    pub fn new() -> UsrCollection {
        let items = vec![];
        UsrCollection { items: items }
    }

    pub async fn get(&self, _ids: Option<Vec<i32>>) -> connectors::Result<Vec<usr::Usr>> {
        Ok(self.items.clone())
    }
}
