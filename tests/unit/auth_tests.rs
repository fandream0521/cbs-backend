use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
};
use std::sync::Arc;

use cbs_backend::auth::{guard_bearer, AuthErrorResponse, AuthState};
use tests::common::test_pool;

#[tokio::test]
async fn test_auth_middleware_valid_token() {
    let state = Arc::new(AuthState {
        allow_any_token: false,
    });
    let req = Request::builder()
        .header(header::AUTHORIZATION, "Bearer demo-token-123")
        .body(Body::empty())
        .unwrap();
    let next = Next::new(|req| async move {
        assert!(req.extensions().get::<String>().is_some());
        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap()
    });

    let result = guard_bearer(State(state), req, next).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status(), StatusCode::OK);
}

#[tokio::test]
async fn test_auth_middleware_missing_token() {
    let state = Arc::new(AuthState {
        allow_any_token: false,
    });
    let req = Request::builder().body(Body::empty()).unwrap();
    let next = Next::new(|_| async {
        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap()
    });

    let result = guard_bearer(State(state), req, next).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, 401);
}

#[tokio::test]
async fn test_auth_middleware_invalid_token() {
    let state = Arc::new(AuthState {
        allow_any_token: false,
    });
    let req = Request::builder()
        .header(header::AUTHORIZATION, "Bearer invalid-token")
        .body(Body::empty())
        .unwrap();
    let next = Next::new(|_| async {
        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap()
    });

    let result = guard_bearer(State(state), req, next).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, 401);
}

#[tokio::test]
async fn test_auth_middleware_allow_any_token() {
    let state = Arc::new(AuthState {
        allow_any_token: true,
    });
    let req = Request::builder().body(Body::empty()).unwrap();
    let next = Next::new(|_| async {
        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap()
    });

    let result = guard_bearer(State(state), req, next).await;
    assert!(result.is_ok());
}
