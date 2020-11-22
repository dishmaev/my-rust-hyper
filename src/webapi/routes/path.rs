pub const HELTH: &str = "/helth";//return uptime is body if alive
pub const SCHEMA: &str = "/schema";//require object_type
pub const ERROR: &str = "/error";//require error_code
pub const STATE: &str = "/state";//require async_command_id

pub const USR_ITEMS: &str = "/usrs";
pub const USR_SIGHN_IN: &str = "/usr/signin";
pub const USR_SIGHN_UP: &str = "/usr/signup";

pub const ROUTER_ROUTE_GET: &str = "/router/route/get";
pub const ROUTER_ROUTE_ADD: &str = "/router/route/add";
pub const ROUTER_ROUTE_REMOVE: &str = "/router/route/remove";
pub const ROUTER_COMMAND_GET: &str = "/router/command/get";
pub const ROUTER_EVENT_GET: &str = "/router/event/get";
pub const ROUTER_SUBSCIBTION_GET: &str = "/router/subscription/get";
pub const ROUTER_SERVICE_GET: &str = "/router/service/get";
pub const ROUTER_EVENT_ON_SERVICE_UNAVAILABLE: &str = "/router/event/on_service_unavailable";

pub const EVENT_ON_ROUTE_UPDATE: &str = "/event/on_route_update";
pub const EVENT_ON_ASYNC_COMMAND_STATE_CHANGE: &str = "/event/on_async_command_state_change";

pub const CAR_GET: &str = "/car/get";
pub const CAR_ADD: &str = "/car/add";
pub const CAR_CHANGE: &str = "/car/change";
pub const CAR_REMOVE: &str = "/car/remove";
pub const CAR_RESERVE: &str = "/car/reserve";

#[cfg(test)]
pub const ROUTE_WITH_EMPTY_BODY: [&str; 4] = [
    HELTH,
    USR_SIGHN_IN,
    USR_SIGHN_UP,
    USR_ITEMS
];
