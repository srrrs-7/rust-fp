use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request};
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

pub const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

pub async fn request_id(mut request: Request<Body>, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();
    request.extensions_mut().insert(request_id.clone());
    let mut response = next.run(request).await;
    let header_value =
        HeaderValue::from_str(&request_id).unwrap_or_else(|_| HeaderValue::from_static("unknown"));
    response
        .headers_mut()
        .insert(REQUEST_ID_HEADER, header_value);
    response
}
