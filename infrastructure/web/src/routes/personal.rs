use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controllers::personal::country::add_verification,
    controllers::personal::persona::{create_persona, get_personas},
    state::AppState,
};

pub fn personal_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/personal/persona", post(create_persona))
        .route("/personal/persona", get(get_personas))
        .route("/personal/country/verification", post(add_verification))
        .with_state(app_state.as_ref().clone())
}
