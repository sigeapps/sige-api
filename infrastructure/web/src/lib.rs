pub mod auth;
pub mod controllers;
pub mod error;
pub mod routes;
pub mod state;
pub mod types;

use std::sync::Arc;

use auth::Backend;
use axum::{http::HeaderValue, routing::get, Router};
use axum_login::AuthManagerLayerBuilder;
use tower_sessions::{cookie::SameSite, SessionManagerLayer};
use error::WebError;
use state::AppState;
use tower_http::cors::CorsLayer;
use tower_sessions::cookie::Key;
use tower_sessions_sqlx_store::PostgresStore;

pub type Result<T, E = WebError> = std::result::Result<T, E>;

#[tokio::main]
pub async fn start(host: &str, port: u16, database_url: &str) -> anyhow::Result<()> {
    let app_state = Arc::new(AppState::new(database_url).await?);

    let pool = application::connection::connect_pool(database_url).await?;

    let key = Key::generate();

    let session_store = PostgresStore::new(pool);

    session_store.migrate().await?;

    let session_layer = SessionManagerLayer::new(session_store)
    .with_same_site(SameSite::None)
    .with_signed(key)
    .with_http_only(true);

    let backend = Backend {
        users: app_state.user_service.clone(),
    };

    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let address = format!("{}:{}", host, port);

    let cors = CorsLayer::very_permissive().allow_origin(
    "http://localhost:1420".parse::<HeaderValue>().unwrap(),
    ).allow_credentials(true);

    let app = Router::new()
        .route("/", get(root))
        .with_state(app_state.clone())
        .merge(routes::auth::auth_routes(&app_state))
        .merge(routes::prevention::prevention_routes(&app_state))
        .merge(routes::lookup::lookup_routes(&app_state))
        .layer(cors)
        .layer(auth_layer);

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Bienvenido a la API de SIGE! Esperando Conexion..."
}
