mod config;
mod database;
mod error;
mod handlers;
mod middleware;
mod models;
mod state;
mod utils;

use axum::{
    Router, middleware as axum_middleware,
    routing::{delete, get, post, put},
};
use state::AppState;
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

    let app_state = AppState::new(pool, config.clone());

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/api/tasks", get(handlers::tasks::list_tasks))
        .route("/api/tasks/{id}", get(handlers::tasks::get_task))
        .route("/api/tasks", post(handlers::tasks::create_task))
        .route("/api/tasks/{id}", put(handlers::tasks::update_task))
        .route("/api/tasks/{id}", delete(handlers::tasks::delete_task))
        .route("/auth/whoami", get(handlers::auth::whoami))
        .layer(axum_middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth::auth_middleware,
        ));

    // Admin routes (require admin role)
    let admin_routes = Router::new()
        .route(
            "/admin/tasks/{id}",
            delete(handlers::tasks::admin_delete_any_task),
        )
        .layer(axum_middleware::from_fn(middleware::auth::admin_middleware))
        .layer(axum_middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth::auth_middleware,
        ));

    // Combine routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start server
    let addr = format!("0.0.0.0:{}", config.server_port);
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.expect("Server error");
}
