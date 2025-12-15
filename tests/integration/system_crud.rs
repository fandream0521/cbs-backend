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
async fn test_user_crud_flow() {
    let (server, token) = setup_test_app().await;

    // Create user
    let create_response = server
        .post("/users")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "name": "newuser",
            "realname": "New User",
            "password": "password123",
            "cellphone": "12345678901"
        }))
        .await;

    create_response.assert_status(StatusCode::OK);
    let create_body: serde_json::Value = create_response.json();
    let user_id = create_body["data"]["id"].as_i64().unwrap();

    // Get user
    let get_response = server
        .get(&format!("/users/{}", user_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    get_response.assert_status(StatusCode::OK);
    let get_body: serde_json::Value = get_response.json();
    assert_eq!(get_body["data"]["name"], "newuser");
    assert_eq!(get_body["data"]["realname"], "New User");

    // Update user
    let update_response = server
        .patch(&format!("/users/{}", user_id))
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "cellphone": "98765432109"
        }))
        .await;

    update_response.assert_status(StatusCode::OK);

    // Verify update
    let get_response2 = server
        .get(&format!("/users/{}", user_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    let get_body2: serde_json::Value = get_response2.json();
    assert_eq!(get_body2["data"]["cellphone"], "98765432109");

    // List users
    let list_response = server
        .post("/users/list")
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

    // Delete user
    let delete_response = server
        .delete(&format!("/users/{}", user_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    delete_response.assert_status(StatusCode::OK);

    // Verify deletion
    let get_response3 = server
        .get(&format!("/users/{}", user_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    get_response3.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_department_crud_flow() {
    let (server, token) = setup_test_app().await;

    // Create department
    let create_response = server
        .post("/department")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "name": "Engineering",
            "leader": "John Doe"
        }))
        .await;

    create_response.assert_status(StatusCode::OK);
    let create_body: serde_json::Value = create_response.json();
    let dept_id = create_body["data"]["id"].as_i64().unwrap();

    // Get department
    let get_response = server
        .get(&format!("/department/{}", dept_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    get_response.assert_status(StatusCode::OK);
    let get_body: serde_json::Value = get_response.json();
    assert_eq!(get_body["data"]["name"], "Engineering");

    // List departments
    let list_response = server
        .post("/department/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 0,
            "size": 10
        }))
        .await;

    list_response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_role_crud_flow() {
    let (server, token) = setup_test_app().await;

    // Create role
    let create_response = server
        .post("/role")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "name": "Manager",
            "intro": "Management role"
        }))
        .await;

    create_response.assert_status(StatusCode::OK);
    let create_body: serde_json::Value = create_response.json();
    let role_id = create_body["data"]["id"].as_i64().unwrap();

    // Get role
    let get_response = server
        .get(&format!("/role/{}", role_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    get_response.assert_status(StatusCode::OK);
    let get_body: serde_json::Value = get_response.json();
    assert_eq!(get_body["data"]["name"], "Manager");
}

#[tokio::test]
async fn test_menu_crud_flow() {
    let (server, token) = setup_test_app().await;

    // Create menu
    let create_response = server
        .post("/menu")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "name": "Dashboard",
            "type": 1,
            "url": "/dashboard",
            "icon": "dashboard-icon",
            "sort": 1
        }))
        .await;

    create_response.assert_status(StatusCode::OK);
    let create_body: serde_json::Value = create_response.json();
    let menu_id = create_body["data"]["id"].as_i64().unwrap();

    // Get menu
    let get_response = server
        .get(&format!("/menu/{}", menu_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    get_response.assert_status(StatusCode::OK);
    let get_body: serde_json::Value = get_response.json();
    assert_eq!(get_body["data"]["name"], "Dashboard");
    assert_eq!(get_body["data"]["type"], 1);
}

#[tokio::test]
async fn test_pagination_validation() {
    let (server, token) = setup_test_app().await;

    // Test invalid size
    let response = server
        .post("/users/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": 0,
            "size": 0
        }))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    // Test negative offset
    let response2 = server
        .post("/users/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "offset": -1,
            "size": 10
        }))
        .await;

    response2.assert_status(StatusCode::BAD_REQUEST);
}
