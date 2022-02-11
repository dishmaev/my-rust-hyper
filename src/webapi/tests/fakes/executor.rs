use super::super::super::{connectors, entities::executor, errors};

pub struct SendedAsyncCommandCollection {
    _items: Vec<executor::SendedAsyncCommand>,
}

impl SendedAsyncCommandCollection {
    pub fn new() -> SendedAsyncCommandCollection {
        let items = vec![];
        SendedAsyncCommandCollection { _items: items }
    }

    pub async fn get(
        &self,
        _ids: Option<Vec<String>>,
    ) -> connectors::Result<Vec<executor::SendedAsyncCommand>> {
        Ok(self._items.clone())
    }
}

pub struct ReceivedAsyncCommandCollection {
    items: Vec<executor::ReceivedAsyncCommand>,
}

impl ReceivedAsyncCommandCollection {
    pub fn new() -> ReceivedAsyncCommandCollection {
        let items = vec![];
        ReceivedAsyncCommandCollection { items: items }
    }

    pub async fn get(
        &self,
        _ids: Option<Vec<String>>,
    ) -> connectors::Result<Vec<executor::ReceivedAsyncCommand>> {
        Ok(self.items.clone())
    }

    pub async fn change_state(
        &self,
        state: String,
        ids: Vec<String>,
    ) -> connectors::Result<(
        errors::ErrorCode,
        Option<Vec<(String, executor::AsyncCommandState)>>,
    )> {
        if state == executor::CommandSystemState::Initial.to_string()
            || state == executor::CommandSystemState::Completed.to_string()
        {
            return Err(errors::UnknownAsyncCommandStateError.into());
        }
        Ok((errors::ErrorCode::ReplyOk, None))
    }

    pub async fn complete(&self, ids: Vec<String>) -> connectors::Result<errors::ErrorCode> {
        Ok(errors::ErrorCode::ReplyOk)
    }
}
