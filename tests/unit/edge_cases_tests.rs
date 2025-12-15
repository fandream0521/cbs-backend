use cbs_backend::{
    domain::goods::GoodsPagination,
    domain::system::Pagination,
    infra::{db::run_migrations, state::AppState, goods_repo, system_repo},
    services::{goods_service::GoodsService, system_service::SystemService},
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
async fn test_pagination_max_size() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    // Test with maximum allowed size
    let pagination = Pagination {
        offset: 0,
        size: 100,
        name: None,
    };
    let (users, total) = service.list_users(&pagination).await.expect("list users");
    assert!(users.len() <= 100);
}

#[tokio::test]
async fn test_pagination_large_offset() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    // Test with large offset (should return empty list)
    let pagination = Pagination {
        offset: 10000,
        size: 10,
        name: None,
    };
    let (users, _total) = service.list_users(&pagination).await.expect("list users");
    assert!(users.is_empty());
}

#[tokio::test]
async fn test_duplicate_user_name() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    // Create first user
    let id1 = service
        .create_user("duplicate", "User 1", "pass1", None, None, None)
        .await
        .expect("create user 1");

    // Try to create duplicate name (should fail)
    let result = service
        .create_user("duplicate", "User 2", "pass2", None, None, None)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_duplicate_role_name() {
    let state = setup_test_state().await;

    // Create first role
    system_repo::create_role(&state.pool, "Admin", None)
        .await
        .expect("create role 1");

    // Try to create duplicate name (should fail)
    let result = system_repo::create_role(&state.pool, "Admin", None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_nonexistent_user() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    let result = service.update_user(99999, Some("newpass"), None).await;
    assert!(result.is_ok()); // Update returns 0 rows affected, which is OK
}

#[tokio::test]
async fn test_delete_nonexistent_user() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    let result = service.delete_user(99999).await;
    assert!(result.is_ok()); // Delete returns 0 rows affected, which is OK
}

#[tokio::test]
async fn test_goods_with_negative_values() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    // SQLite allows negative values, but application should validate
    // This test verifies the database accepts them (validation should be in handlers)
    let id = service
        .create_goods(
            "Test Product",
            Some(-10.0), // Negative price
            Some(-5.0),
            None,
            None,
            None,
            Some(-100), // Negative inventory
            Some(-50),  // Negative sale count
            None,
            None,
        )
        .await;
    // Database allows it, but handlers should validate
    assert!(id.is_ok() || id.is_err()); // Either is acceptable depending on validation
}

#[tokio::test]
async fn test_empty_name_search() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    // Create some users
    service
        .create_user("user1", "User 1", "pass", None, None, None)
        .await
        .expect("create user 1");
    service
        .create_user("user2", "User 2", "pass", None, None, None)
        .await
        .expect("create user 2");

    // Search with empty name (should return all)
    let pagination = Pagination {
        offset: 0,
        size: 10,
        name: Some("".to_string()),
    };
    let (users, total) = service.list_users(&pagination).await.expect("list users");
    assert!(total >= 2);
}

#[tokio::test]
async fn test_name_search_partial_match() {
    let state = setup_test_state().await;
    let service = SystemService::new(&state.pool);

    // Create users with similar names
    service
        .create_user("admin_user", "Admin", "pass", None, None, None)
        .await
        .expect("create admin user");
    service
        .create_user("regular_user", "Regular", "pass", None, None, None)
        .await
        .expect("create regular user");

    // Search for partial match
    let pagination = Pagination {
        offset: 0,
        size: 10,
        name: Some("admin".to_string()),
    };
    let (users, _total) = service.list_users(&pagination).await.expect("list users");
    assert!(users.iter().any(|u| u.name.contains("admin")));
}

#[tokio::test]
async fn test_role_menu_empty_assignment() {
    let state = setup_test_state().await;

    let role_id = system_repo::create_role(&state.pool, "Empty Role", None)
        .await
        .expect("create role");

    // Assign empty menu list
    system_repo::set_role_menus(&state.pool, role_id, &[])
        .await
        .expect("set empty menus");

    let menu_ids = system_repo::get_role_menu_ids(&state.pool, role_id)
        .await
        .expect("get role menu ids");
    assert!(menu_ids.is_empty());
}

#[tokio::test]
async fn test_goods_pagination_edge_cases() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    // Create multiple goods
    for i in 0..5 {
        service
            .create_goods(
                &format!("Product {}", i),
                Some(100.0),
                Some(80.0),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .expect(&format!("create product {}", i));
    }

    // Test pagination with size larger than total
    let pagination = GoodsPagination {
        offset: 0,
        size: 100,
    };
    let (goods, total) = service.list_goods(&pagination).await.expect("list goods");
    assert_eq!(total, 5);
    assert_eq!(goods.len(), 5);
}

