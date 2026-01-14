use rocket::Route;

pub struct RouteGroup {
    pub mount: &'static str,
    pub routes: fn() -> Vec<Route>,
}

inventory::collect!(RouteGroup);

#[macro_export]
macro_rules! register_routes {
    ($mount:expr, $routes:expr) => {
        inventory::submit! {
            $crate::RouteGroup {
                mount: $mount,
                routes: || $routes,
            }
        }
    };
}
