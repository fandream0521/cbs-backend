use axum::http::{header, StatusCode};
use axum_test::TestServer;
use cbs_backend::{
    api::{auth::LoginRequest, router::build_router},
    infra::{db::run_migrations, state::AppState},
};
use serde_json::json;
use std::sync::Arc;
use tests::common::test_pool;

async fn setup_test_app() -> TestServer {
    let pool = test_pool().await;
    run_migrations(&pool).await.expect("run migrations");

    // Create a test user
    sqlx::query(
        r#"
        INSERT INTO users (name, realname, password, enable)
        VALUES ('testuser', 'Test User', 'testpass', 1)
        "#,
    )
    .execute(&pool)
    .await
    .expect("create test user");

    let state = AppState {
        pool: Arc::new(pool),
    };
    let app = build_router(state);
    TestServer::new(app).unwrap()
}

#[tokio::test]
async fn test_login_success() {
    let server = setup_test_app().await;

    let response = server
        .post("/login")
        .json(&json!({
            "name": "testuser",
            "password": "testpass"
        }))
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    assert!(body["data"]["token"].as_str().unwrap().starts_with("Bearer demo-token-"));
    assert_eq!(body["data"]["user"]["name"], "testuser");
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let server = setup_test_app().await;

    let response = server
        .post("/login")
        .json(&json!({
            "name": "testuser",
            "password": "wrongpass"
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 401);
}

#[tokio::test]
async fn test_login_empty_credentials() {
    let server = setup_test_app().await;

    let response = server
        .post("/login")
        .json(&json!({
            "name": "",
            "password": ""
        }))
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_token_validation_success() {
    let server = setup_test_app().await;

    // First login to get token
    let login_response = server
        .post("/login")
        .json(&json!({
            "name": "testuser",
            "password": "testpass"
        }))
        .await;

    let login_body: serde_json::Value = login_response.json();
    let token = login_body["data"]["token"].as_str().unwrap();

    // Test token validation
    let response = server
        .get("/test")
        .add_header(header::AUTHORIZATION, token)
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    assert_eq!(body["data"]["valid"], true);
}

#[tokio::test]
async fn test_protected_route_without_token() {
    let server = setup_test_app().await;

    let response = server.get("/users/list").await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_with_invalid_token() {
    let server = setup_test_app().await;

    let response = server
        .get("/users/list")
        .add_header(header::AUTHORIZATION, "Bearer invalid-token")
        .await;

    response.assert_status(StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_with_valid_token() {
    let server = setup_test_app().await;

    // Login to get token
    let login_response = server
        .post("/login")
        .json(&json!({
            "name": "testuser",
            "password": "testpass"
        }))
        .await;

    let login_body: serde_json::Value = login_response.json();
    let token = login_body["data"]["token"].as_str().unwrap();

    // Access protected route
    let response = server
        .post("/users/list")
        .add_header(header::AUTHORIZATION, token)
        .json(&json!({
            "offset": 0,
            "size": 10
        }))
        .await;

    response.assert_status(StatusCode::OK);
}
