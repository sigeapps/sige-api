use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::controllers::operations::inclusion::*;
use crate::routes::OpenApiRouterExt;

pub fn inclusion_routes() -> OpenApiRouter {
    OpenApiRouter::new().nest_routes(
        "/inclusion",
        routes!(create_inclusion, get_inclusion_summary),
    )
}
