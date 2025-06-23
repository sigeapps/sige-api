use axum::routing::get;
use axum::{routing::post, Router};
use domain::auth::permissions::Permission;

use crate::controllers::user::{create_role, create_user, get_user_by_id, get_users, update_user};
use crate::middleware::authorize;
use crate::state::AppState;
use std::sync::Arc;

pub fn user_routes(app_state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/user", post(create_user).get(get_users))
        .route("/user/{id}", get(get_user_by_id).patch(update_user))
        .route("/role", post(create_role))
        .route_layer(axum::middleware::from_fn_with_state(
            Permission::UsersCreate,
            authorize,
        ))
        .with_state(app_state.as_ref().clone())
}
