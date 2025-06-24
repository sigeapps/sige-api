use std::sync::Arc;

use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    controllers::personal::{
        country::add_verification,
        persona::{create_persona, get_persona, get_personas, update_persona},
    },
    middleware::authenticate,
    state::AppState,
};

pub fn personal_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/personal/persona", post(create_persona))
        .route("/personal/persona", get(get_personas))
        .route("/personal/persona/{id}", get(get_persona))
        .route("/personal/persona/{id}", patch(update_persona))
        .route("/personal/country/verification", post(add_verification))
        .layer(axum::middleware::from_fn(authenticate))
        .with_state(app_state.as_ref().clone())
}
