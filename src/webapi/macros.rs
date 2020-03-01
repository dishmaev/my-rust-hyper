
#[macro_export]
macro_rules! get_ok_reply {
    () => {
        Reply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
        }
    };
}

#[macro_export]
macro_rules! get_ok_add_reply {
    ($ids:expr) => {
        AddReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            ids: Some($ids),
        }
    };
}

#[macro_export]
macro_rules! get_error_reply {
    ($ec:expr, $en:expr) => {
        Reply {
            error_code: $ec,
            error_name: Some($en.get(&($ec as isize)).unwrap().clone()),
        }
    };
}

#[macro_export]
macro_rules! get_error_add_reply {
    ($ec:expr, $en:expr) => {
        AddReply {
            error_code: $ec,
            error_name: Some($en.get(&($ec as isize)).unwrap().clone()),
            ids: None
        }
    };
}
