use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controllers::prevention::register::{create_register, get_registers},
    state::AppState,
};
use std::sync::Arc;

pub fn prevention(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/prevention/register", get(get_registers))
        .route("/prevention/register", post(create_register))
        .route("/prevention/register/:id", post(create_register))
        .with_state(app_state.as_ref().clone())
}
