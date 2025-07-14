use axum::routing::{get, post};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    controllers::prevention::{
        commission::{
            create_commission, get_commission_by_id, get_commission_status_by_id, get_commissions,
            update_commission_exit, update_commission_status,
        },
        official::{create_official, get_officials},
        part::{create_part, get_part_by_id, get_parts, update_part_complete},
        seclusion::{
            add_seclusion_visit, create_seclusion, get_seclusion_by_id, get_seclusions,
            update_seclusion_exit,
        },
        transport::{create_transport, get_tranports},
    },
    middleware::authenticate,
};

use crate::controllers::prevention::register::*;
use crate::routes::OpenApiRouterExt;

pub fn prevention_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest_routes("/register", routes!(get_registers, create_register,))
        .nest_routes(
            "/register",
            routes!(update_register_exit, get_register_by_id),
        )
        .route(
            "/prevention/official",
            get(get_officials).post(create_official),
        )
        .route(
            "/prevention/transport",
            get(get_tranports).post(create_transport),
        )
        .route(
            "/prevention/seclusion",
            get(get_seclusions).post(create_seclusion),
        )
        .route(
            "/prevention/seclusion/{id}",
            get(get_seclusion_by_id).patch(update_seclusion_exit),
        )
        .route(
            "/prevention/seclusion/{id}/visit",
            post(add_seclusion_visit),
        )
        .route(
            "/prevention/commission",
            get(get_commissions).post(create_commission),
        )
        .route("/prevention/commission/{id}", get(get_commission_by_id))
        // TODO: revisa esto
        .route(
            "/prevention/commission/{id}/exit",
            get(get_commission_status_by_id).post(update_commission_exit),
        )
        .route(
            "/prevention/commission/{id}/status",
            get(get_commission_status_by_id).patch(update_commission_status),
        )
        .route("/prevention/part", get(get_parts).post(create_part))
        .route(
            "/prevention/part/{id}",
            get(get_part_by_id).patch(update_part_complete),
        )
        .layer(axum::middleware::from_fn(authenticate))
}
