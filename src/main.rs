use axum::{routing::get, Router, response::IntoResponse};
use reqwest;
use std::net::SocketAddr;

// Route handlers
async fn get_public_ip() -> impl IntoResponse {
    match reqwest::get("https://ipinfo.io").await {
        Ok(resp) => match resp.text().await {
            Ok(info) => info,
            Err(_) => "Failed to read IP".to_string(),
        },
        Err(_) => "Failed to fetch IP".to_string(),
    }
}

async fn get_os_info() -> impl IntoResponse {
    match tokio::fs::read_to_string("/etc/os-release").await {
        Ok(info) => info,
        Err(_) => "Failed to read /etc/os-release".to_string(),
    }
}

// Router setup
fn create_router() -> Router {
    Router::new()
        .route("/info", get(get_public_ip))
        .route("/osinfo", get(get_os_info))
}

// Server setup
async fn setup_server(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(), 
        app
    )
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    let app = create_router();
    setup_server(app).await;
}
