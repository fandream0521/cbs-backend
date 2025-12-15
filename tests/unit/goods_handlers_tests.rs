use cbs_backend::{
    domain::goods::{CategoryPagination, GoodsPagination},
    infra::{db::run_migrations, state::AppState},
    services::goods_service::GoodsService,
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
async fn test_create_goods_success() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    let id = service
        .create_goods(
            "Test Product",
            Some(100.0),
            Some(88.0),
            Some("Test description"),
            Some(1),
            Some("http://example.com/image.png"),
            Some(100),
            Some(50),
            Some(10),
            Some("Beijing"),
        )
        .await
        .expect("create goods");

    assert!(id > 0);
    let goods = service.get_goods(id).await.expect("get goods");
    assert_eq!(goods.name, "Test Product");
    assert_eq!(goods.old_price, Some(100.0));
    assert_eq!(goods.new_price, Some(88.0));
}

#[tokio::test]
async fn test_get_goods_not_found() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    let result = service.get_goods(99999).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        cbs_backend::domain::error::DomainError::NotFound
    ));
}

#[tokio::test]
async fn test_update_goods_success() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    let id = service
        .create_goods("Original", Some(100.0), Some(90.0), None, None, None, None, None, None, None)
        .await
        .expect("create goods");

    service
        .update_goods(id, Some("Updated"), None, Some(80.0), None, None, None, None, None, None, None)
        .await
        .expect("update goods");

    let goods = service.get_goods(id).await.expect("get goods");
    assert_eq!(goods.name, "Updated");
    assert_eq!(goods.new_price, Some(80.0));
}

#[tokio::test]
async fn test_delete_goods_success() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    let id = service
        .create_goods("To Delete", None, None, None, None, None, None, None, None, None)
        .await
        .expect("create goods");

    service.delete_goods(id).await.expect("delete goods");

    let result = service.get_goods(id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_goods_with_pagination() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    // Create test goods
    service
        .create_goods("Product 1", None, None, None, None, None, None, None, None, None)
        .await
        .expect("create goods1");
    service
        .create_goods("Product 2", None, None, None, None, None, None, None, None, None)
        .await
        .expect("create goods2");

    let pagination = GoodsPagination {
        offset: 0,
        size: 10,
    };
    let (goods_list, total) = service.list_goods(&pagination).await.expect("list goods");
    assert!(total >= 2);
    assert!(!goods_list.is_empty());
}

#[tokio::test]
async fn test_create_category_success() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    let id = service
        .create_category("Electronics", None)
        .await
        .expect("create category");

    assert!(id > 0);
    let category = service.get_category(id).await.expect("get category");
    assert_eq!(category.name, "Electronics");
}

#[tokio::test]
async fn test_get_category_not_found() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    let result = service.get_category(99999).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        cbs_backend::domain::error::DomainError::NotFound
    ));
}

#[tokio::test]
async fn test_update_category_success() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    let id = service
        .create_category("Original Category", None)
        .await
        .expect("create category");

    service
        .update_category(id, Some("Updated Category"), None)
        .await
        .expect("update category");

    let category = service.get_category(id).await.expect("get category");
    assert_eq!(category.name, "Updated Category");
}

#[tokio::test]
async fn test_delete_category_success() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    let id = service
        .create_category("To Delete", None)
        .await
        .expect("create category");

    service.delete_category(id).await.expect("delete category");

    let result = service.get_category(id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_categories_with_pagination() {
    let state = setup_test_state().await;
    let service = GoodsService::new(&state.pool);

    // Create test categories
    service
        .create_category("Category 1", None)
        .await
        .expect("create category1");
    service
        .create_category("Category 2", None)
        .await
        .expect("create category2");

    let pagination = CategoryPagination {
        offset: 0,
        size: 10,
        name: None,
    };
    let (categories, total) = service.list_categories(&pagination).await.expect("list categories");
    assert!(total >= 2);
    assert!(!categories.is_empty());
}

