use axum::{middleware, Router};

use crate::middleware::cognito_auth::cognito_auth;
use crate::AppState;

mod tasks;
mod users;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(tasks::router())
        .merge(users::router())
        .layer(middleware::from_fn(cognito_auth))
        .with_state(state)
}
