use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controllers::personal::{
        country::add_verification,
        persona::{create_persona, get_personas},
    },
    middleware::authenticate,
    state::AppState,
};

pub fn personal_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/personal/persona", post(create_persona))
        .route("/personal/persona", get(get_personas))
        .route("/personal/country/verification", post(add_verification))
        .layer(axum::middleware::from_fn(authenticate))
        .with_state(app_state.as_ref().clone())
}
