use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    controllers::prevention::{
        commission::create_commission,
        official::{create_official, get_officials},
        register::{create_register, get_register_by_id, get_registers, update_register_exit},
        transport::{create_transport, get_tranports},
    },
    state::AppState,
};
use std::sync::Arc;

pub fn prevention_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/prevention/register", get(get_registers))
        .route("/prevention/register", post(create_register))
        .route("/prevention/register/{id}", get(get_register_by_id))
        .route("/prevention/register/{id}", patch(update_register_exit))
        .route("/prevention/official", get(get_officials))
        .route("/prevention/official", post(create_official))
        .route("/prevention/transport", get(get_tranports))
        .route("/prevention/transport", post(create_transport))
        .route("/prevention/commission", post(create_commission))
        .with_state(app_state.as_ref().clone())
}
