mod config;
mod database;
mod error;
mod handlers;
mod models;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load config
    let config = config::Config::from_env();

    // Create database pool
    let pool = database::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");
    tracing::info!("Database connected");

    // Build routes
    let app = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/api/tasks", get(handlers::tasks::list_tasks))
        .route("/api/tasks/{id}", get(handlers::tasks::get_task))
        .route("/api/tasks", post(handlers::tasks::create_task))
        .route("/api/tasks/{id}", put(handlers::tasks::update_task))
        .route("/api/tasks/{id}", delete(handlers::tasks::delete_task))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // Start server
    let addr = format!("0.0.0.0:{}", config.server_port);
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.expect("Server error");
}
