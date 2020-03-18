pub const HELTH: &str = "/helth";

pub const ERROR_ITEMS: &str = "/errors";
pub const ERROR_GET: &str = "/error/get";

pub const USR_ITEMS: &str = "/usrs";
pub const USR_SIGHN_IN: &str = "/usr/signin";
pub const USR_SIGHN_UP: &str = "/usr/signup";

pub const ROUTE_ITEMS: &str = "/router/routes";
pub const ROUTE_GET: &str = "/router/route/get";
pub const ROUTE_ADD: &str = "/router/route/add";
pub const ROUTE_REMOVE: &str = "/router/route/remove";
pub const ROUTE_SUBSCIBTION_ITEMS: &str = "/router/subscriptions";
pub const ROUTE_SUBSCIBTION_GET: &str = "/router/subscription/get";
pub const ROUTE_COMMAND_ITEMS: &str = "/router/commands";
pub const ROUTE_COMMAND_GET: &str = "/router/command/get";
pub const ROUTE_EVENT_ON_SERVICE_UNAVAILABLE: &str = "/router/event/on_service_unavailable";
pub const ROUTE_EVENT_ON_ROUTE_UPDATE: &str = "/router/event/on_route_update";

pub const CAR_ITEMS: &str = "/cars";
pub const CAR_GET: &str = "/car/get";
pub const CAR_ADD: &str = "/car/add";
pub const CAR_UPDATE: &str = "/car/update";
pub const CAR_REMOVE: &str = "/car/remove";

#[cfg(test)]
pub const ROUTE_WITH_EMPTY_BODY: [&str; 9] = [
    HELTH,
    USR_SIGHN_IN,
    USR_SIGHN_UP,
    CAR_ITEMS,
    ROUTE_ITEMS,
    ROUTE_SUBSCIBTION_ITEMS,
    ROUTE_COMMAND_ITEMS,
    USR_ITEMS,
    ERROR_ITEMS,
];
