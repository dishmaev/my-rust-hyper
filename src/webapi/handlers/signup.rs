use super::super::{connectors, collections::*, errors, models};

pub async fn signup() -> connectors::Result<models::Reply> {
    Ok(models::Reply {
        error_code: errors::ErrorCode::ReplyOk,
        error_name: None,
    })
}