use axum::{routing::post, Router};

use crate::controllers::auth::{login, register};
use crate::state::AppState;
use std::sync::Arc;

pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(app_state.as_ref().clone())
}
