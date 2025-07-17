use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::controllers::parking::issuance::*;
use crate::controllers::parking::weapon::*;
use crate::middleware::authenticate;
use crate::routes::OpenApiRouterExt;

pub fn parking_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest_routes("/weapons", routes!(get_weapons, create_weapon))
        .nest_routes("/weapons", routes!(get_weapon_by_id))
        .nest_routes("/issuances", routes!(get_issuances, start_issuance))
        .nest_routes("/issuances", routes!(finalize_issuance))
        .nest_routes("/issuances", routes!(get_issuance_by_id))
        .layer(axum::middleware::from_fn(authenticate))
}
