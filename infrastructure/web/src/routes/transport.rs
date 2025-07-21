use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::routes::OpenApiRouterExt;

use crate::controllers::transport::issuance::*;

pub fn transport_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest_routes("/issuances", routes!(get_issuances, start_issuance))
        .nest_routes("/issuances", routes!(finalize_issuance))
        .nest_routes("/issuances", routes!(get_issuance_by_id))
}
