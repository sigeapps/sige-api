use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::controllers::parking::weapon::*;
use crate::middleware::authenticate;
use crate::routes::OpenApiRouterExt;

pub fn parking_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest_routes("/weapons", routes!(get_weapon_by_id, create_weapon))
        .layer(axum::middleware::from_fn(authenticate))
}
