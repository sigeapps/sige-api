use axum::routing::get;
use axum::routing::post;
use domain::auth::permissions::Permission;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::controllers::user::*;
use crate::middleware::authorize;
use crate::routes::OpenApiRouterExt;

pub fn user_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/user", post(create_user).get(get_users))
        .route("/user/{id}", get(get_user_by_id).patch(update_user))
        .nest_routes("/permission", routes!(get_permissions))
        .nest_routes("/role", routes!(create_role))
        .route_layer(axum::middleware::from_fn_with_state(
            Permission::UsersCreate,
            authorize,
        ))
}
