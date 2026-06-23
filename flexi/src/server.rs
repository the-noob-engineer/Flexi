use axum::{Router, routing::get};

pub async fn run_server(port: Option<u16>) {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let ip_host = format!("0.0.0.0:{}", port.unwrap_or(7259));

    log::info!("Server listening on {}", ip_host);
    let listener = tokio::net::TcpListener::bind(ip_host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
