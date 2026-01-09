use axum::http::{HeaderName, HeaderValue, Request, Response};
use axum::middleware::Next;
use uuid::Uuid;

pub const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

pub async fn request_id<B>(mut request: Request<B>, next: Next<B>) -> Response {
    let request_id = Uuid::new_v4().to_string();
    request.extensions_mut().insert(request_id.clone());
    let mut response = next.run(request).await;
    let header_value =
        HeaderValue::from_str(&request_id).unwrap_or_else(|_| HeaderValue::from_static("unknown"));
    response.headers_mut().insert(REQUEST_ID_HEADER, header_value);
    response
}
