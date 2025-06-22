pub mod auth;
pub mod controllers;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod state;
pub mod types;

use std::sync::Arc;

use axum::{routing::get, Router};
use error::WebError;
use state::AppState;
use tower_http::cors::CorsLayer;

pub type Result<T, E = WebError> = std::result::Result<T, E>;

#[tokio::main]
pub async fn start(host: &str, port: u16, database_url: &str) -> anyhow::Result<()> {
    let app_state = Arc::new(AppState::new(database_url).await?);

    let address = format!("{}:{}", host, port);

    let cors = CorsLayer::very_permissive();

    let app = Router::new()
        .route("/", get(root))
        .with_state(app_state.clone())
        .merge(routes::user::user_routes(&app_state))
        .merge(routes::auth::auth_routes(&app_state))
        .merge(routes::prevention::prevention_routes(&app_state))
        .merge(routes::personal::personal_routes(&app_state))
        .merge(routes::lookup::lookup_routes(&app_state))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Bienvenido a la API de SIGE! Esperando Conexion..."
}
