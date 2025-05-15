use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controllers::lookup::{create_brigade, get_brigades},
    state::AppState,
};
use std::sync::Arc;

pub fn lookup_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/lookup/brigade", get(get_brigades))
        .route("/lookup/brigade", post(create_brigade))
        .with_state(app_state.as_ref().clone())
}
