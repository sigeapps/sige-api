use axum::routing::get;
use axum::{routing::post, Router};
use domain::auth::permissions::Permission;

use crate::controllers::user::{create_role, create_user, get_user_by_id, get_users, update_user};
use crate::middleware::authorize;

pub fn user_routes() -> Router {
    Router::new()
        .route("/user", post(create_user).get(get_users))
        .route("/user/{id}", get(get_user_by_id).patch(update_user))
        .route("/role", post(create_role))
        .route_layer(axum::middleware::from_fn_with_state(
            Permission::UsersCreate,
            authorize,
        ))
}
