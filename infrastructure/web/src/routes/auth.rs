use axum::routing::get;
use axum::{routing::post, Router};

use crate::controllers::auth::{get_current_user, login};
use crate::middleware::authenticate;

pub fn auth_routes() -> Router {
    Router::new().route("/login", post(login)).route(
        "/me",
        get(get_current_user).layer(axum::middleware::from_fn(authenticate)),
    )
}
