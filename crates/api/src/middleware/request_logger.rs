use axum::http::{Request, Response};
use axum::middleware::Next;
use std::time::Instant;

pub async fn request_logger<B>(request: Request<B>, next: Next<B>) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let request_id = request
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());

    let start = Instant::now();
    let response = next.run(request).await;
    let duration_ms = start.elapsed().as_millis();

    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        status = %response.status(),
        duration_ms = duration_ms,
        "request completed"
    );

    response
}
