use super::{events, handlers, commands};

pub trait ObjectType {
    fn get_type_name() -> &'static str;
}

impl ObjectType for handlers::models::Reply {
    fn get_type_name() -> &'static str {
        "Reply"
    }
}

impl ObjectType for handlers::models::AddIntIdsReply {
    fn get_type_name() -> &'static str {
        "AddIntIdsReply"
    }
}

impl ObjectType for handlers::models::AddStrIdsReply {
    fn get_type_name() -> &'static str {
        "AddStrIdsReply"
    }
}

impl ObjectType for events::route::OnRouteUpdate {
    fn get_type_name() -> &'static str {
        "OnRouteUpdate"
    }
}

impl ObjectType for events::route::OnServiceUnavailable {
    fn get_type_name() -> &'static str {
        "OnServiceUnavailable"
    }
}

impl ObjectType for commands::car::MoveCar {
    fn get_type_name() -> &'static str {
        "MoveCar"
    }
}
