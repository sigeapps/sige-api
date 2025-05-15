use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controllers::lookup::{
        create_brigade, create_charge, create_division, create_hierarchy, create_organism,
        get_brigades, get_charges, get_divisions, get_hierarchies, get_organisms,
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
        .with_state(app_state.as_ref().clone())
}
