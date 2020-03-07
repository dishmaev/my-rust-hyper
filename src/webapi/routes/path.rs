pub const ROUTE_SIGHN_IN: &str = "/api/signin";
pub const ROUTE_SIGHN_UP: &str = "/api/signup";

pub const ROUTE_SUBSCRIPTION_ITEMS: &str = "/subscriptions";
pub const ROUTE_SUBSCRIPTION_GET: &str = "/subscription/get";

pub const ROUTE_CAR_ON_DELETE_SUBSCRIBE: &str = "/car/ondelete/subscribe";
pub const ROUTE_CAR_ON_DELETE_UNSUBSCRIBE: &str = "/car/ondelete/unsubscribe";

pub const ROUTE_CAR_ITEMS: &str = "/cars";
pub const ROUTE_CAR_GET: &str = "/car/get";
pub const ROUTE_CAR_ADD: &str = "/car/add";
pub const ROUTE_CAR_UPDATE: &str = "/car/update";
pub const ROUTE_CAR_DELETE: &str = "/car/delete";

pub const ROUTE_USR_ITEMS: &str = "/usrs";

#[cfg(test)]
pub const ROUTES: [&str; 7] = [
    ROUTE_SIGHN_IN,
    ROUTE_SIGHN_UP,
    ROUTE_CAR_GET,
    ROUTE_CAR_DELETE,
    ROUTE_CAR_ON_DELETE_SUBSCRIBE,
    ROUTE_CAR_ON_DELETE_UNSUBSCRIBE,
    ROUTE_SUBSCRIPTION_ITEMS,
];

