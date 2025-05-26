pub mod auth;
pub mod controllers;
pub mod error;
pub mod routes;
pub mod state;
pub mod types;

use std::sync::Arc;

use auth::Backend;
use axum::{routing::get, Router};
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use error::WebError;
use password_auth::generate_hash;
use state::AppState;
use tower_http::cors::CorsLayer;

pub type Result<T, E = WebError> = std::result::Result<T, E>;

#[tokio::main]
pub async fn start(host: &str, port: u16, database_url: &str) -> anyhow::Result<()> {
    let app_state = Arc::new(AppState::new(database_url).await?);

    let password = generate_hash("taller");
    println!("{}", password);

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = Backend {
        users: app_state.user_service.clone(),
    };

    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let address = format!("{}:{}", host, port);

    let app = Router::new()
        .route("/", get(root))
        .with_state(app_state.clone())
        .merge(routes::auth::auth_routes(&app_state))
        .merge(routes::prevention::prevention_routes(&app_state))
        .merge(routes::lookup::lookup_routes(&app_state))
        .layer(CorsLayer::very_permissive())
        .layer(auth_layer);

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Welcome to SIGE API!"
}
