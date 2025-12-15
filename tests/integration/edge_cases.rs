use axum::http::{header, StatusCode};
use axum_test::TestServer;
use cbs_backend::{
    api::router::build_router,
    infra::{db::run_migrations, state::AppState},
};
use serde_json::json;
use std::sync::Arc;
use tests::common::test_pool;

async fn setup_test_app() -> (TestServer, String, Arc<sqlx::SqlitePool>) {
    let pool = test_pool().await;
    run_migrations(&pool).await.expect("run migrations");

    // Create a test user for auth
    sqlx::query(
        r#"
        INSERT INTO users (name, realname, password, enable)
        VALUES ('admin', 'Admin User', 'admin123', 1)
        "#,
    )
    .execute(&pool)
    .await
    .expect("create test user");

    let pool_arc = Arc::new(pool);
    let state = AppState {
        pool: pool_arc.clone(),
    };
    let app = build_router(state);
    let server = TestServer::new(app).unwrap();

    // Login to get token
    let login_response = server
        .post("/login")
        .json(&json!({
            "name": "admin",
            "password": "admin123"
        }))
        .await;

    let login_body: serde_json::Value = login_response.json();
    let token = login_body["data"]["token"].as_str().unwrap().to_string();

    (server, token, pool_arc)
}

#[tokio::test]
async fn test_pagination_edge_cases() {
    let (server, token, _pool) = setup_test_app().await;

    // Test maximum size
    let response = server
        .post("/users/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 0,
            "size": 100
        }))
        .await;

    response.assert_status(StatusCode::OK);

    // Test size exceeds limit
    let response2 = server
        .post("/users/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 0,
            "size": 101
        }))
        .await;

    response2.assert_status(StatusCode::BAD_REQUEST);

    // Test large offset
    let response3 = server
        .post("/users/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 10000,
            "size": 10
        }))
        .await;

    response3.assert_status(StatusCode::OK);
    let body: serde_json::Value = response3.json();
    assert_eq!(body["data"]["totalCount"].as_i64().unwrap(), 1); // Only admin user
}

#[tokio::test]
async fn test_sql_injection_prevention() {
    let (server, token, _pool) = setup_test_app().await;

    // Test SQL injection attempt in name filter
    let response = server
        .post("/users/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 0,
            "size": 10,
            "name": "admin'; DROP TABLE users; --"
        }))
        .await;

    // Should not crash, should treat as literal string
    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert!(body["data"]["list"].is_array());
}

#[tokio::test]
async fn test_very_long_strings() {
    let (server, token, _pool) = setup_test_app().await;

    // Test very long title/content
    let long_string = "a".repeat(10000);
    let response = server
        .post("/story")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "title": long_string.clone(),
            "content": long_string
        }))
        .await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_unicode_and_special_chars() {
    let (server, token, _pool) = setup_test_app().await;

    // Test unicode characters
    let response = server
        .post("/story")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "title": "测试标题 🎉",
            "content": "内容包含特殊字符: <>&\"'"
        }))
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    let story_id = body["data"]["id"].as_i64().unwrap();

    // Verify retrieval
    let list_response = server
        .post("/story/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({}))
        .await;

    list_response.assert_status(StatusCode::OK);
    let list_body: serde_json::Value = list_response.json();
    let stories = list_body["data"]["list"].as_array().unwrap();
    let story = stories.iter().find(|s| s["id"].as_i64().unwrap() == story_id).unwrap();
    assert_eq!(story["title"], "测试标题 🎉");
}

#[tokio::test]
async fn test_concurrent_requests() {
    let (server, token, _pool) = setup_test_app().await;

    // Create multiple concurrent requests
    let mut handles = Vec::new();
    for i in 0..10 {
        let server_clone = server.clone();
        let token_clone = token.clone();
        handles.push(tokio::spawn(async move {
            server_clone
                .post("/story")
                .add_header(header::AUTHORIZATION, &token_clone)
                .json(&json!({
                    "title": format!("Story {}", i),
                    "content": format!("Content {}", i)
                }))
                .await
        }));
    }

    // Wait for all requests
    for handle in handles {
        let response = handle.await.unwrap();
        response.assert_status(StatusCode::OK);
    }

    // Verify all stories were created
    let list_response = server
        .post("/story/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({}))
        .await;

    list_response.assert_status(StatusCode::OK);
    let list_body: serde_json::Value = list_response.json();
    assert!(list_body["data"]["totalCount"].as_i64().unwrap() >= 10);
}

#[tokio::test]
async fn test_missing_required_fields() {
    let (server, token, _pool) = setup_test_app().await;

    // Test missing name in goods
    let response = server
        .patch("/goods")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "oldPrice": 100.0
        }))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    // Test missing title in story
    let response2 = server
        .post("/story")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "content": "Content"
        }))
        .await;

    response2.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_invalid_id_formats() {
    let (server, token, _pool) = setup_test_app().await;

    // Test non-existent ID
    let response = server
        .get("/users/99999")
        .add_header(header::AUTHORIZATION, &token)
        .await;

    response.assert_status(StatusCode::NOT_FOUND);

    // Test negative ID (should be handled gracefully)
    let response2 = server
        .get("/users/-1")
        .add_header(header::AUTHORIZATION, &token)
        .await;

    // Should either return 404 or 400
    assert!(response2.status() == StatusCode::NOT_FOUND || response2.status() == StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_empty_arrays_and_lists() {
    let (server, token, _pool) = setup_test_app().await;

    // Test empty menu list assignment
    let role_id = 1; // Assuming role exists or will be created
    let response = server
        .post("/role/assign")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "roleId": role_id,
            "menuList": []
        }))
        .await;

    // Should succeed (clearing menus)
    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_whitespace_handling() {
    let (server, token, _pool) = setup_test_app().await;

    // Test whitespace-only strings
    let response = server
        .post("/story")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "title": "   ",
            "content": "   "
        }))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    // Test strings with leading/trailing whitespace (should be trimmed)
    let response2 = server
        .post("/story")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "title": "  Valid Title  ",
            "content": "  Valid Content  "
        }))
        .await;

    response2.assert_status(StatusCode::OK);
}

