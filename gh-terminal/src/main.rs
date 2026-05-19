mod api_routes;
mod assets;
mod config;
mod handler;
mod serial_port;
mod state;
mod ws;

use std::sync::Arc;
use clap::Parser;
use tracing::info;

#[tokio::main]
async fn main() {
    let config = config::Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(&config.log_filter)
        .init();

    info!("Starting GH-Terminal...");

    let app_state = Arc::new(state::AppState::new());

    // 启动串口任务
    let serial_state = app_state.clone();
    let serial_config = config.clone();
    tokio::spawn(async move {
        serial_port::run(serial_state, &serial_config).await;
    });

    // 启动 HTTP/WS 服务
    let router = api_routes::build_router(app_state);
    let addr = format!("0.0.0.0:{}", config.http_port);
    info!("HTTP server listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
