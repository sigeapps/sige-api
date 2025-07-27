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
        .route("/personal/persona", post(create_persona))
        .route("/personal/persona", get(get_personas))
        .nest_routes("/personal/persona/summary", routes!(create_persona_summary))
        .route("/personal/persona/{id}", get(get_persona))
        .route("/personal/persona/{id}", patch(update_persona))
        .route(
            "/personal/persona/{id}/traits",
            patch(update_traits).layer(authorize!(Permission::PersonasUpdateTraits)),
        )
        .route(
            "/personal/persona/{id}/academic",
            post(add_education).layer(authorize!(Permission::PersonasUpdateAcademic)),
        )
        .route(
            "/personal/persona/{id}/courses",
            post(add_courses).layer(authorize!(Permission::PersonasUpdateCourses)),
        )
        .route(
            "/personal/persona/{id}/labor",
            post(add_work_experience).layer(authorize!(Permission::PersonasUpdateLabor)),
        )
        .route(
            "/personal/persona/{id}/operational",
            post(add_operational).layer(authorize!(Permission::PersonasUpdateLabor)),
        )
        .route(
            "/personal/persona/{id}/records",
            post(add_records).layer(authorize!(Permission::PersonasUpdateRecords)),
        )
        .route(
            "/personal/persona/{id}/health",
            patch(update_health).layer(authorize!(Permission::PersonasUpdateHealth)),
        )
        .route(
            "/personal/persona/{id}/situation",
            patch(update_situation).layer(authorize!(Permission::PersonasUpdateSituation)),
        )
        .route(
            "/personal/persona/{id}/summary",
            patch(update_persona_summary),
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
