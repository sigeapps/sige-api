use axum::routing::get;
use axum::{routing::post, Router};

use crate::controllers::auth::{get_current_user, login};
use crate::middleware::authenticate;
use crate::state::AppState;
use std::sync::Arc;

pub fn auth_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login))
        .route(
            "/me",
            get(get_current_user).layer(axum::middleware::from_fn(authenticate)),
        )
        .with_state(app_state.as_ref().clone())
}
