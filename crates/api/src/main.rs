use axum::{routing::get, Router};
use std::sync::Arc;

use application::task_service::repository::TaskRepository;
use application::user_service::repository::UserRepository;
use infrastructure::db::{build_pool, DbConfig};
use infrastructure::task_repo::repository::TaskRepositoryImpl;
use infrastructure::user_repo::repository::UserRepositoryImpl;
use tracing_subscriber::{fmt, EnvFilter};

mod middleware;
mod response;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub task_repo: Arc<dyn TaskRepository>,
    pub user_repo: Arc<dyn UserRepository>,
}

#[tokio::main]
async fn main() {
    init_tracing();

    let config = DbConfig::from_env().expect("Missing database configuration");
    let pool = build_pool(&config)
        .await
        .expect("Failed to build database pool");

    let state = AppState {
        task_repo: Arc::new(TaskRepositoryImpl::new(pool.clone())),
        user_repo: Arc::new(UserRepositoryImpl::new(pool)),
    };

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/v1", routes::router::router(state))
        .layer(axum::middleware::from_fn(
            middleware::request_logger::request_logger,
        ))
        .layer(axum::middleware::from_fn(
            middleware::request_id::request_id,
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind listener");

    tracing::info!("Server listening on http://localhost:8080");
    axum::serve(listener, app).await.expect("Server failed");
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).json().init();
}
