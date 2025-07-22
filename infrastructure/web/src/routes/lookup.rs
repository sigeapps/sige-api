use axum::{
    routing::{get, post},
    Router,
};

use crate::{controllers::lookup::*, middleware::authenticate};

pub fn lookup_routes() -> Router {
    Router::new()
        .route("/lookup/brigade", get(get_brigades))
        .route("/lookup/brigade", post(create_brigade))
        .route("/lookup/organism", get(get_organisms))
        .route("/lookup/organism", post(create_organism))
        .route("/lookup/division", get(get_divisions))
        .route("/lookup/division", post(create_division))
        .route("/lookup/charge", get(get_charges))
        .route("/lookup/charge", post(create_charge))
        .route("/lookup/hierarchy", get(get_hierarchies))
        .route("/lookup/hierarchy", post(create_hierarchy))
        .route("/lookup/state", get(get_states))
        .route("/lookup/state", post(create_state))
        .route("/lookup/municipality", get(get_municipalities))
        .route("/lookup/municipality", post(create_municipality))
        .route("/lookup/transport-type", get(get_transport_types))
        .route("/lookup/transport-type", post(create_transport_type))
        .route("/lookup/transport-status", get(get_transport_statuses))
        .route("/lookup/transport-status", post(create_transport_status))
        .route("/lookup/brand", get(get_brands))
        .route("/lookup/brand", post(create_brand))
        .route("/lookup/vehicle-model", get(get_vehicle_models))
        .route("/lookup/vehicle-model", post(create_vehicle_model))
        .route("/lookup/parish", get(get_parish))
        .route("/lookup/parish", post(create_parish))
        .route("/lookup/novelty", get(get_novelties))
        .route("/lookup/status-conditions", get(get_status_conditions))
        .route("/lookup/novelty", post(create_novelty))
        .route("/lookup/seclusion-status", get(get_seclusion_statuses))
        .route("/lookup/band", get(get_bands))
        .route("/lookup/band", post(create_band))
        .route("/lookup/institution", get(get_institutions))
        .route("/lookup/institution", post(create_institution))
        .route("/lookup/profession", get(get_professions))
        .route("/lookup/profession", post(create_profession))
        .route(
            "/lookup/seclusion-status",
            post(create_family_relationships),
        )
        .route("/lookup/family-relationship", get(get_family_relationships))
        .route(
            "/lookup/family-relationship",
            post(create_seclusion_statuses),
        )
        .route("/lookup/base", get(get_bases))
        .route("/lookup/role", get(get_roles))
        .route("/lookup/base", post(create_base))
        .route("/lookup/persona-state", get(get_persona_states))
        .route("/lookup/persona-state", post(create_persona_state))
        .route("/lookup/document-type", get(get_document_types))
        .route("/lookup/document-type", post(create_document_type))
        .route(
            "/lookup/weapon-type",
            post(create_weapon_type).get(get_weapon_types),
        )
        .route(
            "/lookup/weapon-brand",
            post(create_weapon_brand).get(get_weapon_brands),
        )
        .route(
            "/lookup/weapon-model",
            post(create_weapon_model).get(get_weapon_models),
        )
        .route(
            "/lookup/assignance-time",
            post(create_assignance_time).get(get_assignance_times),
        )
        .route("/lookup/calibre", post(create_calibre).get(get_calibres))
        .route("/lookup/position", post(create_position).get(get_positions))
        .layer(axum::middleware::from_fn(authenticate))
}
