pub mod auth;
pub mod controllers;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod tags;
pub mod types;

use application::api::ApiContext;
use application::connection::connect;
use moka::future::Cache;
use axum::http::{HeaderName, HeaderValue, Method};
use axum::routing::get;
use axum::Extension;
use error::WebError;
use routes::parking::parking_routes;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::compression::CompressionLayer;

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::router::OpenApiRouter;

#[cfg(debug_assertions)]
use utoipa_swagger_ui::SwaggerUi;


use crate::routes::transport::transport_routes;

pub type Result<T, E = WebError> = std::result::Result<T, E>;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components = Some(
            utoipa::openapi::ComponentsBuilder::new()
                .security_scheme(
                    "api_jwt_token",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                )
                .build(),
        )
    }
}

#[tokio::main]
pub async fn start(host: &str, port: u16, db_url: &str) -> anyhow::Result<()> {
    let db = connect(db_url).await?;
    let cache = Cache::builder()
        .max_capacity(10_000)
        .time_to_live(std::time::Duration::from_secs(3600)) // 1 hora de vida por defecto
        .build();

    let address = format!("{host}:{port}");

    // CORS temporalmente abierto para depuración, NO USAR EN PRODUCCIÓN
    // TODO: Agregar toda esta configuracion a settings.json
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:1420".parse::<HeaderValue>().unwrap(),
            "http://localhost:1421".parse::<HeaderValue>().unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://localhost:1420".parse::<HeaderValue>().unwrap(),
            "http://tauri.localhost".parse::<HeaderValue>().unwrap(),
            "https://tauri.localhost".parse::<HeaderValue>().unwrap(),
            "http://app.tecnoelectronics.com.ve"
                .parse::<HeaderValue>()
                .unwrap(),
            "https://app.tecnoelectronics.com.ve"
                .parse::<HeaderValue>()
                .unwrap(),
            "http://192.168.1.101:8443".parse::<HeaderValue>().unwrap(),
            "http://localhost:8443".parse::<HeaderValue>().unwrap(),
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
        .allow_credentials(true);

    let (router, _api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/", get(root))
        .nest("/parking", parking_routes())
        .nest("/transport", transport_routes())
        .merge(routes::user::user_routes())
        .merge(routes::auth::auth_routes())
        .merge(routes::inclusion::inclusion_routes())
        .merge(routes::prevention::prevention_routes())
        .merge(routes::personal::personal_routes().into())
        .merge(routes::lookup::lookup_routes().into())
        .layer(Extension(ApiContext {
            db,
            claims: None,
            cache,
        }))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
                tracing::info_span!(
                    "http",
                    method = %request.method(),
                    uri = %request.uri(),
                    user = tracing::field::Empty,
                )
            }),
        )
        .layer(CompressionLayer::new())
        .split_for_parts();

    let app = router;

    #[cfg(debug_assertions)]
    let app = app
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", _api.clone()));


    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Bienvenido a la API de SIGE! Esperando Conexion..."
}
