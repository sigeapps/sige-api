use axum::routing::{get, patch, post};
use domain::auth::permissions::Permission;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    authorize,
    controllers::personal::{
        country::*,
        persona::*,
        plate::{create_plate, get_plate_by_id, get_plates},
        process::{create_correspondence, get_correspondence, get_correspondences},
    },
    middleware::{authenticate, authorize},
    routes::OpenApiRouterExt,
};

pub fn personal_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest_routes("/personal/persona", routes!(create_persona, get_personas))
        .nest_routes("/personal/persona", routes!(get_persona))
        .nest_routes("/personal/persona/{id}/traits", routes!(create_traits))
        .nest_routes("/personal/persona/{id}/health", routes!(create_health))
        .nest_routes(
            "/personal/persona/{id}/situation",
            routes!(create_situation),
        )
        .nest_routes("/personal/persona/{id}/spouse", routes!(create_spouse))
        .nest_routes("/personal/persona/{id}/children", routes!(create_child))
        .nest_routes(
            "/personal/persona/{id}/operational",
            routes!(create_operational),
        )
        .nest_routes("/personal/persona/{id}/relatives", routes!(create_relative))
        .nest_routes(
            "/personal/persona/{id}/education",
            routes!(create_education),
        )
        .nest_routes("/personal/persona/{id}/courses", routes!(create_course))
        .nest_routes(
            "/personal/persona/{id}/work-experience",
            routes!(create_work_experience),
        )
        .nest_routes("/personal/persona/{id}/records", routes!(create_record))
        .route(
            "/personal/persona/{id}/traits",
            patch(update_traits).layer(authorize!(Permission::PersonasUpdateTraits)),
        )
        .route(
            "/personal/persona/{id}/health",
            patch(update_health).layer(authorize!(Permission::PersonasUpdateHealth)),
        )
        .route(
            "/personal/persona/{id}/situation",
            patch(update_situation).layer(authorize!(Permission::PersonasUpdateSituation)),
        )
        .route("/personal/plate", post(create_plate).get(get_plates))
        .route("/personal/plate/{id}", get(get_plate_by_id))
        .route("/personal/country/verification", post(add_verification))
        .nest_routes("/personal/country/csv", routes!(get_country_csv))
        .route(
            "/personal/correspondence",
            post(create_correspondence).get(get_correspondences),
        )
        .route("/personal/correspondence/{id}", get(get_correspondence))
        .layer(axum::middleware::from_fn(authenticate))
}
