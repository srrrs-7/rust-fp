use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::time::Instant;

pub async fn request_logger(request: Request<Body>, next: Next) -> Response {
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
