use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    // If false, require a valid bearer token.
    pub allow_any_token: bool,
}

pub async fn guard_bearer(
    State(state): State<Arc<AuthState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AuthErrorResponse> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(parse_bearer);

    if !state.allow_any_token {
        match token.as_deref() {
            Some(val) if val.starts_with("demo-token-") => {
                // Valid token format: demo-token-{id}
            }
            _ => return Err(AuthErrorResponse::unauthorized("missing or invalid token")),
        }
    }

    req.extensions_mut().insert(token.unwrap_or_default());
    Ok(next.run(req).await)
}

fn parse_bearer(value: &str) -> Option<String> {
    const PREFIX: &str = "Bearer ";
    if value.starts_with(PREFIX) {
        let rest = &value[PREFIX.len()..];
        if !rest.is_empty() {
            return Some(rest.to_string());
        }
    }
    None
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthErrorResponse {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

impl AuthErrorResponse {
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            code: 401,
            message: message.into(),
            data: None,
        }
    }
}

impl IntoResponse for AuthErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, Json(self)).into_response()
    }
}
