use rocket::{Build, Config, Rocket, Route};
use std::net::{IpAddr, Ipv4Addr};

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

pub fn build_rocket(port: u16) -> Rocket<Build> {
    let config = Config {
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        port,
        ..Config::default()
    };

    let rocket = rocket::custom(config);
    mount_routes(rocket)
}

pub fn mount_routes(mut rocket: Rocket<Build>) -> Rocket<Build> {
    for group in inventory::iter::<RouteGroup> {
        rocket = rocket.mount(group.mount, (group.routes)());
    }

    rocket
}
