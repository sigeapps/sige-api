use utoipa_axum::router::{OpenApiRouter, UtoipaMethodRouterExt};
use utoipa_axum::routes;

use crate::controllers::auth::*;
use crate::middleware::authenticate;

pub fn auth_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(get_current_user).layer(axum::middleware::from_fn(authenticate)))
}
