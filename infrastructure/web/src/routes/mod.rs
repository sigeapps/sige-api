pub mod auth;
pub mod lookup;
pub mod parking;
pub mod personal;
pub mod prevention;
pub mod user;

use utoipa_axum::router::{OpenApiRouter, UtoipaMethodRouter};

pub trait OpenApiRouterExt {
    fn nest_routes(self, path: &str, method: UtoipaMethodRouter) -> Self;
}

impl OpenApiRouterExt for OpenApiRouter {
    fn nest_routes(self, path: &str, method: UtoipaMethodRouter) -> Self {
        let router = OpenApiRouter::new().routes(method);

        self.nest(path, router)
    }
}
