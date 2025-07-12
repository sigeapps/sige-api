use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    controllers::personal::{
        country::add_verification,
        persona::{
            create_persona, get_persona, get_personas, update_persona, update_persona_summary,
        },
        plate::{create_plate, get_plate_by_id, get_plates},
        process::{create_correspondence, get_correspondence, get_correspondences},
    },
    middleware::authenticate,
};

pub fn personal_routes() -> Router {
    Router::new()
        .route("/personal/persona", post(create_persona))
        .route("/personal/persona", get(get_personas))
        .route("/personal/persona/{id}", get(get_persona))
        .route("/personal/persona/{id}", patch(update_persona))
        .route(
            "/personal/persona/{id}/summary",
            patch(update_persona_summary),
        )
        .route("/personal/plate", post(create_plate).get(get_plates))
        .route("/personal/plate/{id}", get(get_plate_by_id))
        .route("/personal/country/verification", post(add_verification))
        .route(
            "/personal/correspondence",
            post(create_correspondence).get(get_correspondences),
        )
        .route("/personal/correspondence/{id}", get(get_correspondence))
        .layer(axum::middleware::from_fn(authenticate))
}
