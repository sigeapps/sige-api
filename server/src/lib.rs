use axum::{routing::get, Router};

#[tokio::main]
pub async fn start(host: &str, port: u16, database_url: &str) {
    let address = format!("{}:{}", host, port);

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to SIGE API!"
}
