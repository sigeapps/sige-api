use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{controllers::personal::persona::create_persona, state::AppState};

pub fn personal_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/personal/persona", post(create_persona))
        .with_state(app_state.as_ref().clone())
}
