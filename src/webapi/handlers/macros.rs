
#[macro_export]
macro_rules! get_ok_reply {
    () => {
        models::Reply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
        }
    };
}

#[macro_export]
macro_rules! get_ok_add_int_ids_reply {
    ($ids:expr) => {
        models::AddIntIdsReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            ids: Some($ids),
        }
    };
}

#[macro_export]
macro_rules! get_ok_add_str_ids_reply {
    ($ids:expr) => {
        models::AddStrIdsReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            ids: Some($ids),
        }
    };
}

#[macro_export]
macro_rules! get_error_reply {
    ($ec:expr, $en:expr) => {
        models::Reply {
            error_code: $ec.clone(),
            error_name: Some($en.get(&$ec.as_isize()).unwrap().clone()),
        }
    };
}

#[macro_export]
macro_rules! get_error_add_int_ids_reply {
    ($ec:expr, $en:expr) => {
        models::AddIntIdsReply {
            error_code: $ec.clone(),
            error_name: Some($en.get(&$ec.as_isize()).unwrap().clone()),
            ids: None
        }
    };
}

#[macro_export]
macro_rules! get_error_add_str_ids_reply {
    ($ec:expr, $en:expr) => {
        models::AddStrIdsReply {
            error_code: $ec.clone(),
            error_name: Some($en.get(&$ec.as_isize()).unwrap().clone()),
            ids: None
        }
    };
}
