use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    controllers::prevention::register::{
        create_register, get_register_by_id, get_registers, update_register_exit,
    },
    state::AppState,
};
use std::sync::Arc;

pub fn prevention(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/prevention/register", get(get_registers))
        .route("/prevention/register", post(create_register))
        .route("/prevention/register/{id}", get(get_register_by_id))
        .route("/prevention/register/{id}", patch(update_register_exit))
        .with_state(app_state.as_ref().clone())
}
