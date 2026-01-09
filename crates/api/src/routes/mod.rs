use axum::{middleware, Router};

use crate::middleware::cognito_auth::cognito_auth;
use crate::AppState;

pub mod tasks;
pub mod users;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/task", axum::routing::post(tasks::post::handler))
        .route("/task/:id", axum::routing::get(tasks::get::handler))
        .route("/task/:id", axum::routing::put(tasks::put::handler))
        .route("/task/:id", axum::routing::delete(tasks::delete::handler))
        .route("/tasks", axum::routing::get(tasks::list::handler))
        .route("/user", axum::routing::post(users::post::handler))
        .route("/user/:id", axum::routing::get(users::get::handler))
        .route("/user/:id", axum::routing::put(users::put::handler))
        .route("/user/:id", axum::routing::delete(users::delete::handler))
        .route("/users", axum::routing::get(users::list::handler))
        .route("/users/by-email/:email", axum::routing::get(users::get_by_email::handler))
        .route(
            "/users/by-username/:username",
            axum::routing::get(users::get_by_username::handler),
        )
        .layer(middleware::from_fn(cognito_auth))
        .with_state(state)
}
