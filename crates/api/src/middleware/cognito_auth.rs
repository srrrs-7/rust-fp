use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use domain::error::AppError;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use tokio::sync::OnceCell;

use crate::response::{from_app_error, ErrorResponse};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub groups: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize, Clone)]
struct Jwk {
    kid: String,
    kty: String,
    n: String,
    e: String,
}

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    email: Option<String>,
    #[serde(rename = "cognito:username")]
    cognito_username: Option<String>,
    #[serde(rename = "cognito:groups")]
    cognito_groups: Option<Vec<String>>,
    token_use: String,
    iss: String,
    aud: Option<String>,
}

static JWKS_CACHE: OnceCell<JwkSet> = OnceCell::const_new();

async fn fetch_jwks() -> Result<JwkSet, AppError> {
    let jwks_uri = std::env::var("COGNITO_JWKS_URI").map_err(|_| AppError::Api {
        message: "COGNITO_JWKS_URI is not set".to_string(),
    })?;

    let client = reqwest::Client::new();
    let jwks = client
        .get(jwks_uri)
        .send()
        .await
        .map_err(|error| AppError::Api {
            message: error.to_string(),
        })?
        .json::<JwkSet>()
        .await
        .map_err(|error| AppError::Api {
            message: error.to_string(),
        })?;

    Ok(jwks)
}

async fn get_jwks() -> Result<&'static JwkSet, AppError> {
    JWKS_CACHE.get_or_try_init(fetch_jwks).await
}

fn build_validation() -> Result<Validation, AppError> {
    let issuer = std::env::var("COGNITO_ISSUER").map_err(|_| AppError::Api {
        message: "COGNITO_ISSUER is not set".to_string(),
    })?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer]);

    if let Ok(aud) = std::env::var("COGNITO_CLIENT_ID") {
        validation.set_audience(&[aud]);
    }

    Ok(validation)
}

fn find_decoding_key(jwks: &JwkSet, kid: &str) -> Result<DecodingKey, AppError> {
    let jwk = jwks
        .keys
        .iter()
        .find(|key| key.kid == kid && key.kty == "RSA")
        .ok_or_else(|| AppError::Unauthorized {
            resource: "Cognito".to_string(),
            message: "Unknown key id".to_string(),
        })?;

    DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|error| AppError::Api {
            message: error.to_string(),
        })
}

pub async fn cognito_auth<B>(mut request: Request<B>, next: Next<B>) -> Result<Response, ErrorResponse> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    if !auth_header.starts_with("Bearer ") {
        return Err(ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED",
            "Authorization header must be Bearer token",
        ));
    }

    let token = auth_header.trim_start_matches("Bearer ").trim();
    if token.is_empty() {
        return Err(ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED",
            "Bearer token is required",
        ));
    }

    let header = decode_header(token)
        .map_err(|_| ErrorResponse::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "Invalid token"))?;

    let kid = header
        .kid
        .ok_or_else(|| ErrorResponse::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "Missing kid"))?;

    let jwks = get_jwks().await.map_err(from_app_error)?;
    let key = find_decoding_key(jwks, &kid).map_err(from_app_error)?;
    let validation = build_validation().map_err(from_app_error)?;

    let token_data = decode::<Claims>(token, &key, &validation)
        .map_err(|_| ErrorResponse::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "Invalid token"))?;

    if token_data.claims.token_use != "access" {
        return Err(ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED",
            "Invalid token type",
        ));
    }

    let user = AuthUser {
        user_id: token_data.claims.sub,
        email: token_data.claims.email,
        username: token_data.claims.cognito_username,
        groups: token_data.claims.cognito_groups.unwrap_or_default(),
    };

    request.extensions_mut().insert(user);
    request.extensions_mut().insert(token.to_string());

    Ok(next.run(request).await)
}
