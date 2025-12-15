use axum::http::{header, StatusCode};
use axum_test::TestServer;
use cbs_backend::{
    api::router::build_router,
    infra::{db::run_migrations, state::AppState},
};
use serde_json::json;
use std::sync::Arc;
use tests::common::test_pool;

async fn setup_test_app() -> (TestServer, String) {
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

    let state = AppState {
        pool: Arc::new(pool),
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

    (server, token)
}

#[tokio::test]
async fn test_goods_crud_flow() {
    let (server, token) = setup_test_app().await;

    // Create goods (using PATCH as per spec)
    let create_response = server
        .patch("/goods")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "name": "Test Product",
            "oldPrice": 100.0,
            "newPrice": 88.0,
            "desc": "Test description",
            "status": 1,
            "imgUrl": "http://example.com/image.png",
            "inventoryCount": 100,
            "saleCount": 50,
            "favorCount": 10,
            "address": "Beijing"
        }))
        .await;

    create_response.assert_status(StatusCode::OK);
    let create_body: serde_json::Value = create_response.json();
    let goods_id = create_body["data"]["id"].as_i64().unwrap();

    // Get goods
    let get_response = server
        .get(&format!("/goods/{}", goods_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    get_response.assert_status(StatusCode::OK);
    let get_body: serde_json::Value = get_response.json();
    assert_eq!(get_body["data"]["name"], "Test Product");
    assert_eq!(get_body["data"]["oldPrice"], 100.0);
    assert_eq!(get_body["data"]["newPrice"], 88.0);

    // Update goods
    let update_response = server
        .patch(&format!("/goods/{}", goods_id))
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "name": "Updated Product",
            "newPrice": 75.0
        }))
        .await;

    update_response.assert_status(StatusCode::OK);

    // Verify update
    let get_response2 = server
        .get(&format!("/goods/{}", goods_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    let get_body2: serde_json::Value = get_response2.json();
    assert_eq!(get_body2["data"]["name"], "Updated Product");
    assert_eq!(get_body2["data"]["newPrice"], 75.0);

    // List goods
    let list_response = server
        .post("/goods/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 0,
            "size": 10
        }))
        .await;

    list_response.assert_status(StatusCode::OK);
    let list_body: serde_json::Value = list_response.json();
    assert!(list_body["data"]["totalCount"].as_i64().unwrap() > 0);
    assert!(list_body["data"]["list"].is_array());

    // Delete goods
    let delete_response = server
        .delete(&format!("/goods/{}", goods_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    delete_response.assert_status(StatusCode::OK);

    // Verify deletion
    let get_response3 = server
        .get(&format!("/goods/{}", goods_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    get_response3.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_category_crud_flow() {
    let (server, token) = setup_test_app().await;

    // Create category
    let create_response = server
        .post("/category")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "name": "Electronics"
        }))
        .await;

    create_response.assert_status(StatusCode::OK);
    let create_body: serde_json::Value = create_response.json();
    let category_id = create_body["data"]["id"].as_i64().unwrap();

    // Get category
    let get_response = server
        .get(&format!("/category/{}", category_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    get_response.assert_status(StatusCode::OK);
    let get_body: serde_json::Value = get_response.json();
    assert_eq!(get_body["data"]["name"], "Electronics");

    // Update category
    let update_response = server
        .patch(&format!("/category/{}", category_id))
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "name": "Updated Electronics"
        }))
        .await;

    update_response.assert_status(StatusCode::OK);

    // Verify update
    let get_response2 = server
        .get(&format!("/category/{}", category_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    let get_body2: serde_json::Value = get_response2.json();
    assert_eq!(get_body2["data"]["name"], "Updated Electronics");

    // List categories
    let list_response = server
        .post("/category/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 0,
            "size": 10,
            "name": ""
        }))
        .await;

    list_response.assert_status(StatusCode::OK);
    let list_body: serde_json::Value = list_response.json();
    assert!(list_body["data"]["totalCount"].as_i64().unwrap() > 0);
    assert!(list_body["data"]["list"].is_array());

    // Delete category
    let delete_response = server
        .delete(&format!("/category/{}", category_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    delete_response.assert_status(StatusCode::OK);

    // Verify deletion
    let get_response3 = server
        .get(&format!("/category/{}", category_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    get_response3.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_goods_pagination_validation() {
    let (server, token) = setup_test_app().await;

    // Test invalid size
    let response = server
        .post("/goods/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 0,
            "size": 0
        }))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    // Test negative offset
    let response2 = server
        .post("/goods/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": -1,
            "size": 10
        }))
        .await;

    response2.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_category_name_filter() {
    let (server, token) = setup_test_app().await;

    // Create categories
    server
        .post("/category")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({ "name": "Electronics" }))
        .await;
    server
        .post("/category")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({ "name": "Clothing" }))
        .await;

    // List with name filter
    let list_response = server
        .post("/category/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 0,
            "size": 10,
            "name": "Elect"
        }))
        .await;

    list_response.assert_status(StatusCode::OK);
    let list_body: serde_json::Value = list_response.json();
    let list = list_body["data"]["list"].as_array().unwrap();
    assert!(list.iter().any(|item| item["name"].as_str().unwrap().contains("Elect")));
}

