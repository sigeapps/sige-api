use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controllers::lookup::{
        create_band, create_base, create_brand, create_brigade, create_charge, create_division,
        create_family_relationships, create_hierarchy, create_institution, create_municipality,
        create_novelty, create_organism, create_parish, create_profession,
        create_seclusion_statuses, create_state, create_transport_status, create_transport_type,
        create_vehicle_model, get_bands, get_bases, get_brands, get_brigades, get_charges,
        get_divisions, get_family_relationships, get_hierarchies, get_institutions,
        get_municipalities, get_novelties, get_organisms, get_parish, get_professions,
        get_seclusion_statuses, get_states, get_status_conditions, get_transport_statuses,
        get_transport_types, get_vehicle_models,
    },
    state::AppState,
};
use std::sync::Arc;

pub fn lookup_routes(app_state: &Arc<AppState>) -> Router {
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
        .route("/lookup/base", post(create_base))
        .with_state(app_state.as_ref().clone())
}
