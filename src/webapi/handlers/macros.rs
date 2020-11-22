#[macro_export]
macro_rules! get_ok_reply {
    () => {
        replies::common::StandardReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
        }
    };
}

#[macro_export]
macro_rules! get_async_ok_reply {
    () => {
        replies::common::StandardReply {
            error_code: errors::ErrorCode::AsyncOk,
            error_name: None,
        }
    };
}

#[macro_export]
macro_rules! get_ok_reply_events {
    ($events:expr) => {
        (
            replies::common::StandardReply {
                error_code: errors::ErrorCode::ReplyOk,
                error_name: None,
            },
            $events,
        )
    };
}

#[macro_export]
macro_rules! get_ok_add_int_ids_reply {
    ($ids:expr) => {
        replies::common::AddIntIdsReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            ids: Some($ids),
        }
    };
}

#[macro_export]
macro_rules! get_ok_add_int_ids_reply_events {
    ($ids:expr, $events:expr) => {
        (
            replies::common::AddIntIdsReply {
                error_code: errors::ErrorCode::ReplyOk,
                error_name: None,
                ids: Some($ids),
            },
            $events,
        )
    };
}

#[macro_export]
macro_rules! get_ok_add_str_ids_reply {
    ($ids:expr) => {
        replies::common::AddStrIdsReply {
            error_code: errors::ErrorCode::ReplyOk,
            error_name: None,
            ids: Some($ids),
        }
    };
}

#[macro_export]
macro_rules! get_ok_add_str_ids_reply_events {
    ($ids:expr, $events:expr) => {
        (
            replies::common::AddStrIdsReply {
                error_code: errors::ErrorCode::ReplyOk,
                error_name: None,
                ids: Some($ids),
            },
            $events,
        )
    };
}

#[macro_export]
macro_rules! get_error_reply {
    ($ec:expr, $en:expr) => {
        replies::common::StandardReply {
            error_code: $ec.clone(),
            error_name: Some($en.get(&$ec.to_string()).unwrap().clone()),
        }
    };
}

#[macro_export]
macro_rules! get_error_reply_events {
    ($ec:expr, $en:expr) => {
        (
            replies::common::StandardReply {
                error_code: $ec.clone(),
                error_name: Some($en.get(&$ec.to_string()).unwrap().clone()),
            },
            None,
        )
    };
}

#[macro_export]
macro_rules! get_error_add_int_ids_reply {
    ($ec:expr, $en:expr) => {
        replies::common::AddIntIdsReply {
            error_code: $ec.clone(),
            error_name: Some($en.get(&$ec.to_string()).unwrap().clone()),
            ids: None,
        }
    };
}

#[macro_export]
macro_rules! get_error_add_int_ids_reply_events {
    ($ec:expr, $en:expr) => {
        (
            replies::common::AddIntIdsReply {
                error_code: $ec.clone(),
                error_name: Some($en.get(&$ec.to_string()).unwrap().clone()),
                ids: None,
            },
            None,
        )
    };
}

#[macro_export]
macro_rules! get_error_add_str_ids_reply {
    ($ec:expr, $en:expr) => {
        (
            replies::common::AddStrIdsReply {
                error_code: $ec.clone(),
                error_name: Some($en.get(&$ec.to_string()).unwrap().clone()),
                ids: None,
            },
            None,
        )
    };
}

#[macro_export]
macro_rules! get_error_add_str_ids_reply_events {
    ($ec:expr, $en:expr) => {
        replies::common::AddStrIdsReply {
            error_code: $ec.clone(),
            error_name: Some($en.get(&$ec.as_isize()).unwrap().clone()),
            ids: None,
        }
    };
}
