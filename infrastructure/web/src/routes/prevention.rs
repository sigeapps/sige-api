use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    controllers::prevention::{
        commission::{
            create_commission, get_commission_by_id, get_commission_status,
            get_commission_status_by_id, get_commissions, update_commission_exit,
            update_commission_status,
        },
        official::{create_official, get_officials},
        part::{create_part, get_part_by_id, get_parts, update_part_complete},
        register::{create_register, get_register_by_id, get_registers, update_register_exit},
        seclusion::{
            add_seclusion_visit, create_seclusion, get_seclusion_by_id, get_seclusions,
            update_seclusion_exit,
        },
        transport::{create_transport, get_tranports},
    },
    middleware::authenticate,
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
        .route("/prevention/seclusion", get(get_seclusions))
        .route("/prevention/seclusion", post(create_seclusion))
        .route("/prevention/seclusion/{id}", get(get_seclusion_by_id))
        .route("/prevention/seclusion/{id}", patch(update_seclusion_exit))
        .route(
            "/prevention/seclusion/{id}/visit",
            post(add_seclusion_visit),
        )
        .route("/prevention/commission", post(create_commission))
        .route("/prevention/commission", get(get_commissions))
        .route(
            "/prevention/commission/{id}/exit",
            get(get_commission_status),
        )
        .route(
            "/prevention/commission/{id}/exit",
            post(update_commission_exit),
        )
        .route(
            "/prevention/commission/{id}/status",
            patch(update_commission_status),
        )
        .route(
            "/prevention/commission/{id}/status",
            get(get_commission_status_by_id),
        )
        .route("/prevention/commission/{id}", get(get_commission_by_id))
        .route("/prevention/part", get(get_parts))
        .route("/prevention/part", post(create_part))
        .route("/prevention/part/{id}", get(get_part_by_id))
        .route("/prevention/part/{id}", patch(update_part_complete))
        .layer(axum::middleware::from_fn(authenticate))
        .with_state(app_state.as_ref().clone())
}
