use axum::http::{header, StatusCode};
use axum_test::TestServer;
use cbs_backend::{
    api::router::build_router,
    infra::{db::run_migrations, state::AppState},
};
use serde_json::json;
use std::sync::Arc;
use tests::common::test_pool;
use sqlx::SqlitePool;

async fn setup_test_app() -> (TestServer, String, Arc<SqlitePool>) {
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
async fn test_story_create_and_list() {
    let (server, token, _pool) = setup_test_app().await;

    // Create story
    let create_response = server
        .post("/story")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "title": "Test Story",
            "content": "This is a test story content"
        }))
        .await;

    create_response.assert_status(StatusCode::OK);
    let create_body: serde_json::Value = create_response.json();
    let story_id = create_body["data"]["id"].as_i64().unwrap();
    assert!(story_id > 0);

    // List stories
    let list_response = server
        .post("/story/list")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({}))
        .await;

    list_response.assert_status(StatusCode::OK);
    let list_body: serde_json::Value = list_response.json();
    assert!(list_body["data"]["totalCount"].as_i64().unwrap() > 0);
    assert!(list_body["data"]["list"].is_array());
    
    let stories = list_body["data"]["list"].as_array().unwrap();
    assert!(stories.iter().any(|s| s["id"].as_i64().unwrap() == story_id));
}

#[tokio::test]
async fn test_story_create_validation() {
    let (server, token, _pool) = setup_test_app().await;

    // Test empty title
    let response = server
        .post("/story")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "title": "",
            "content": "Content"
        }))
        .await;

    response.assert_status(StatusCode::BAD_REQUEST);

    // Test empty content
    let response2 = server
        .post("/story")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "title": "Title",
            "content": ""
        }))
        .await;

    response2.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_menu_tree_endpoint() {
    let (server, token, pool) = setup_test_app().await;

    // Create test menus using the shared pool
    use cbs_backend::infra::system_repo;
    
    let menu1_id = system_repo::create_menu(
        &pool,
        "Menu 1",
        1,
        Some("/menu1"),
        Some("icon1"),
        Some(1),
        None,
    )
    .await
    .expect("create menu 1");

    let _menu2_id = system_repo::create_menu(
        &pool,
        "Menu 2",
        2,
        Some("/menu2"),
        Some("icon2"),
        Some(2),
        Some(menu1_id),
    )
    .await
    .expect("create menu 2");

    // Get menu tree
    let response = server
        .post("/menu/tree")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({}))
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    assert!(body["data"].is_array());
    
    let tree = body["data"].as_array().unwrap();
    assert!(!tree.is_empty());
}

#[tokio::test]
async fn test_role_menu_endpoints() {
    let (server, token, pool) = setup_test_app().await;

    // Setup: Create role and menus using the shared pool
    use cbs_backend::infra::system_repo;
    
    let role_id = system_repo::create_role(&pool, "Test Role", None)
        .await
        .expect("create role");

    let menu_id = system_repo::create_menu(
        &pool,
        "Test Menu",
        1,
        Some("/test"),
        None,
        Some(1),
        None,
    )
    .await
    .expect("create menu");

    system_repo::set_role_menus(&pool, role_id, &[menu_id])
        .await
        .expect("set role menus");

    // Get role menu IDs
    let response = server
        .get(&format!("/role/{}/menuIds", role_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    let menu_ids = body["data"]["menuIds"].as_array().unwrap();
    assert_eq!(menu_ids.len(), 1);
    assert_eq!(menu_ids[0].as_i64().unwrap(), menu_id);

    // Get role menu tree
    let response2 = server
        .get(&format!("/role/{}/menu", role_id))
        .add_header(header::AUTHORIZATION, &token)
        .await;

    response2.assert_status(StatusCode::OK);
    let body2: serde_json::Value = response2.json();
    assert_eq!(body2["code"], 200);
    assert!(body2["data"].is_array());
}

#[tokio::test]
async fn test_assign_role_menus() {
    let (server, token, pool) = setup_test_app().await;

    // Setup: Create role and menus using the shared pool
    use cbs_backend::infra::system_repo;
    
    let role_id = system_repo::create_role(&pool, "Role", None)
        .await
        .expect("create role");

    let menu1_id = system_repo::create_menu(
        &pool,
        "Menu 1",
        1,
        None,
        None,
        Some(1),
        None,
    )
        .await
        .expect("create menu 1");

    let menu2_id = system_repo::create_menu(
        &pool,
        "Menu 2",
        1,
        None,
        None,
        Some(2),
        None,
    )
        .await
        .expect("create menu 2");

    // Assign menus to role
    let response = server
        .post("/role/assign")
        .add_header(header::AUTHORIZATION, &token)
        .json(&json!({
            "roleId": role_id,
            "menuList": [menu1_id, menu2_id]
        }))
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    let menu_ids = body["data"]["menuIds"].as_array().unwrap();
    assert_eq!(menu_ids.len(), 2);
}

#[tokio::test]
async fn test_metrics_category_counts() {
    let (server, token, pool) = setup_test_app().await;

    // Setup: Create categories using the shared pool
    use cbs_backend::infra::goods_repo;
    
    goods_repo::create_category(&pool, "Category 1", None)
        .await
        .expect("create category 1");
    goods_repo::create_category(&pool, "Category 2", None)
        .await
        .expect("create category 2");

    // Get category counts
    let response = server
        .get("/goods/category/count")
        .add_header(header::AUTHORIZATION, &token)
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    assert!(body["data"].is_array());
}

#[tokio::test]
async fn test_metrics_category_sales() {
    let (server, token, _pool) = setup_test_app().await;

    let response = server
        .get("/goods/category/sale")
        .add_header(header::AUTHORIZATION, &token)
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    assert!(body["data"].is_array());
}

#[tokio::test]
async fn test_metrics_category_favors() {
    let (server, token, _pool) = setup_test_app().await;

    let response = server
        .get("/goods/category/favor")
        .add_header(header::AUTHORIZATION, &token)
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    assert!(body["data"].is_array());
}

#[tokio::test]
async fn test_metrics_top_sales() {
    let (server, token, pool) = setup_test_app().await;

    // Setup: Create test goods using the shared pool
    use cbs_backend::infra::goods_repo;
    
    goods_repo::create_goods(
        &pool,
        "Product 1",
        Some(100.0),
        Some(80.0),
        None,
        Some(1),
        None,
        Some(100),
        Some(50),
        Some(10),
        None,
    )
    .await
    .expect("create goods");

    // Get top sales
    let response = server
        .get("/goods/sale/top10")
        .add_header(header::AUTHORIZATION, &token)
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    assert!(body["data"].is_array());
    let top_sales = body["data"].as_array().unwrap();
    assert!(top_sales.len() <= 10);
}

#[tokio::test]
async fn test_metrics_address_sales() {
    let (server, token, pool) = setup_test_app().await;

    // Setup: Create test goods with addresses using the shared pool
    use cbs_backend::infra::goods_repo;
    
    goods_repo::create_goods(
        &pool,
        "Product 1",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("Beijing"),
    )
    .await
    .expect("create goods 1");

    goods_repo::create_goods(
        &pool,
        "Product 2",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("Shanghai"),
    )
    .await
    .expect("create goods 2");

    // Get address sales
    let response = server
        .get("/goods/address/sale")
        .add_header(header::AUTHORIZATION, &token)
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    assert!(body["data"].is_array());
    let address_sales = body["data"].as_array().unwrap();
    assert!(address_sales.len() >= 2);
}

#[tokio::test]
async fn test_metrics_goods_amount_list() {
    let (server, token, _pool) = setup_test_app().await;

    let response = server
        .get("/goods/amount/list")
        .add_header(header::AUTHORIZATION, &token)
        .await;

    response.assert_status(StatusCode::OK);
    let body: serde_json::Value = response.json();
    assert_eq!(body["code"], 200);
    assert!(body["data"].is_array());
}

