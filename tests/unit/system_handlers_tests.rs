use axum::extract::State;
use cbs_backend::{
    api::system::{CreateUserBody, Pagination},
    domain::system::User,
    infra::{db::run_migrations, state::AppState},
    services::system_service::SystemService,
};
use std::sync::Arc;
use tests::common::test_pool;

async fn setup_test_state() -> AppState {
    let pool = test_pool().await;
    run_migrations(&pool).await.expect("run migrations");
    AppState {
        pool: Arc::new(pool),
    }
}

#[tokio::test]
async fn test_create_user_success() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    let id = service
        .create_user("testuser", "Test User", "password123", None, None, None)
        .await
        .expect("create user");

    assert!(id > 0);
    let user = service.get_user(id).await.expect("get user");
    assert_eq!(user.name, "testuser");
    assert_eq!(user.realname, "Test User");
}

#[tokio::test]
async fn test_get_user_not_found() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    let result = service.get_user(99999).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        cbs_backend::domain::error::DomainError::NotFound
    ));
}

#[tokio::test]
async fn test_update_user_success() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    let id = service
        .create_user("updateuser", "Update User", "oldpass", None, None, None)
        .await
        .expect("create user");

    service
        .update_user(id, Some("newpass"), Some("12345678901"))
        .await
        .expect("update user");

    let user = service.get_user(id).await.expect("get user");
    assert_eq!(user.cellphone, Some("12345678901".to_string()));
}

#[tokio::test]
async fn test_delete_user_success() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    let id = service
        .create_user("deleteuser", "Delete User", "pass", None, None, None)
        .await
        .expect("create user");

    service.delete_user(id).await.expect("delete user");

    let result = service.get_user(id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_users_with_pagination() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    // Create test users
    service
        .create_user("user1", "User 1", "pass", None, None, None)
        .await
        .expect("create user1");
    service
        .create_user("user2", "User 2", "pass", None, None, None)
        .await
        .expect("create user2");

    let pagination = Pagination {
        offset: 0,
        size: 10,
        name: None,
    };
    let (users, total) = service.list_users(&pagination).await.expect("list users");
    assert!(total >= 2);
    assert!(!users.is_empty());
}

#[tokio::test]
async fn test_create_department_success() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    let id = service
        .create_department("Engineering", None, Some("John Doe"))
        .await
        .expect("create department");

    assert!(id > 0);
    let dept = service.get_department(id).await.expect("get department");
    assert_eq!(dept.name, "Engineering");
    assert_eq!(dept.leader, Some("John Doe".to_string()));
}

#[tokio::test]
async fn test_create_role_success() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    let id = service
        .create_role("Admin", Some("Administrator role"))
        .await
        .expect("create role");

    assert!(id > 0);
    let role = service.get_role(id).await.expect("get role");
    assert_eq!(role.name, "Admin");
    assert_eq!(role.intro, Some("Administrator role".to_string()));
}

#[tokio::test]
async fn test_create_menu_success() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    let id = service
        .create_menu("Dashboard", 1, Some("/dashboard"), Some("icon"), Some(1), None)
        .await
        .expect("create menu");

    assert!(id > 0);
    let menu = service.get_menu(id).await.expect("get menu");
    assert_eq!(menu.name, "Dashboard");
    assert_eq!(menu.r#type, 1);
}
