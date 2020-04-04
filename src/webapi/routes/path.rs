pub const HELTH: &str = "/helth";
pub const SCHEMA: &str = "/schema";
pub const ERROR: &str = "/error";

pub const USR_ITEMS: &str = "/usrs";
pub const USR_SIGHN_IN: &str = "/usr/signin";
pub const USR_SIGHN_UP: &str = "/usr/signup";

pub const ROUTE_GET: &str = "/router/route/get";
pub const ROUTE_ADD: &str = "/router/route/add";
pub const ROUTE_REMOVE: &str = "/router/route/remove";
pub const ROUTE_COMMAND_GET: &str = "/router/command/get";
pub const ROUTE_EVENT_GET: &str = "/router/event/get";
pub const ROUTE_SUBSCIBTION_GET: &str = "/router/subscription/get";
pub const ROUTE_SERVICE_GET: &str = "/route/service/get";
pub const ROUTE_EVENT_ON_SERVICE_UNAVAILABLE: &str = "/router/event/on_service_unavailable";

pub const EVENT_ON_ROUTE_UPDATE: &str = "/event/on_route_update";

pub const CAR_GET: &str = "/car/get";
pub const CAR_ADD: &str = "/car/add";
pub const CAR_MODIFY: &str = "/car/modify";
pub const CAR_REMOVE: &str = "/car/remove";
pub const CAR_RESERVE: &str = "/car/reserve";

#[cfg(test)]
pub const ROUTE_WITH_EMPTY_BODY: [&str; 4] = [
    HELTH,
    USR_SIGHN_IN,
    USR_SIGHN_UP,
    USR_ITEMS
];
