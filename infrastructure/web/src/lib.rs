pub mod auth;
use axum::{routing::get, Router};

#[tokio::main]
pub async fn start(host: &str, port: u16, database_url: &str) -> anyhow::Result<()> {
    let address = format!("{}:{}", host, port);

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Welcome to SIGE API!"
}
