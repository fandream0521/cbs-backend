use axum::{Json, extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    api::ApiResponse,
    auth::AuthErrorResponse,
    infra::{state::AppState, system_repo},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
    pub id: i64,
    pub name: String,
    pub realname: String,
    pub cellphone: String,
    pub enable: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
    pub user: LoginUser,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthErrorResponse> {
    if payload.name.is_empty() || payload.password.is_empty() {
        return Err(AuthErrorResponse::unauthorized("invalid credentials"));
    }

    let db_user =
        system_repo::find_user_by_credentials(&state.pool, &payload.name, &payload.password)
            .await
            .map_err(|_| AuthErrorResponse::unauthorized("authentication failed"))?;

    let user = match db_user {
        Some(u) => u,
        None => return Err(AuthErrorResponse::unauthorized("invalid credentials")),
    };

    // Simple token: in production, use JWT or similar
    let token = format!("Bearer demo-token-{}", user.id);
    let login_user = LoginUser {
        id: user.id,
        name: user.name,
        realname: user.realname,
        cellphone: user.cellphone.unwrap_or_default(),
        enable: user.enable,
    };
    let resp = ApiResponse::success(LoginResponse {
        token,
        user: login_user,
    });
    Ok(Json(resp))
}

pub async fn test_token(State(_state): State<AppState>) -> impl IntoResponse {
    // Token validation is handled by middleware; if we reach here, token is valid.
    let resp = ApiResponse::success(serde_json::json!({ "valid": true }));
    Json(resp)
}
