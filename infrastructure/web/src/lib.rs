pub mod auth;
pub mod controllers;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod types;

use application::api::ApiContext;
use application::connection::connect;
use axum::http::{HeaderName, HeaderValue, Method};
use axum::Extension;
use axum::{routing::get, Router};
use error::WebError;
use tower_http::cors::CorsLayer;

pub type Result<T, E = WebError> = std::result::Result<T, E>;

#[tokio::main]
pub async fn start(host: &str, port: u16, db_url: &str) -> anyhow::Result<()> {
    let db = connect(db_url).await?;

    let address = format!("{}:{}", host, port);

    // Configuración específica de CORS - Corregida para evitar el error
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:1420".parse::<HeaderValue>().unwrap(),
            "http://localhost:1421".parse::<HeaderValue>().unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://localhost:1420".parse::<HeaderValue>().unwrap(),
            "http://tauri.localhost".parse::<HeaderValue>().unwrap(),
            "http://app.tecnoelectronics.com.ve"
                .parse::<HeaderValue>()
                .unwrap(),
            "https://app.tecnoelectronics.com.ve"
                .parse::<HeaderValue>()
                .unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            "accept".parse::<HeaderName>().unwrap(),
            "accept-language".parse::<HeaderName>().unwrap(),
            "authorization".parse::<HeaderName>().unwrap(),
            "content-type".parse::<HeaderName>().unwrap(),
            "dnt".parse::<HeaderName>().unwrap(),
            "origin".parse::<HeaderName>().unwrap(),
            "user-agent".parse::<HeaderName>().unwrap(),
            "x-csrftoken".parse::<HeaderName>().unwrap(),
            "x-requested-with".parse::<HeaderName>().unwrap(),
        ])
        .allow_credentials(true)
        .expose_headers([
            "content-length".parse::<HeaderName>().unwrap(),
            "content-range".parse::<HeaderName>().unwrap(),
        ]);

    let app = Router::new()
        .route("/", get(root))
        .merge(routes::user::user_routes())
        .merge(routes::auth::auth_routes())
        .merge(routes::prevention::prevention_routes())
        .merge(routes::personal::personal_routes())
        .merge(routes::lookup::lookup_routes())
        .layer(Extension(ApiContext { db, claims: None }))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Bienvenido a la API de SIGE! Esperando Conexion..."
}
