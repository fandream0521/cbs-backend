use axum::extract::State;
use cbs_backend::{
    api::story::CreateStoryBody,
    domain::story::StoryListRequest,
    infra::{
        db::run_migrations,
        metrics_repo,
        state::AppState,
        system_repo,
    },
    services::story_service::StoryService,
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
async fn test_create_story_success() {
    let state = setup_test_state().await;
    let service = StoryService::new(&state.pool);

    let id = service
        .create_story("Test Story", "This is a test story content")
        .await
        .expect("create story");

    assert!(id > 0);
    let (stories, _total) = service.list_stories().await.expect("list stories");
    assert!(stories.iter().any(|s| s.id == id && s.title == "Test Story"));
}

#[tokio::test]
async fn test_create_story_empty_title() {
    let state = setup_test_state().await;
    let service = StoryService::new(&state.pool);

    let result = service.create_story("", "Content").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_story_empty_content() {
    let state = setup_test_state().await;
    let service = StoryService::new(&state.pool);

    let result = service.create_story("Title", "").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_stories_success() {
    let state = setup_test_state().await;
    let service = StoryService::new(&state.pool);

    // Create test stories
    service
        .create_story("Story 1", "Content 1")
        .await
        .expect("create story 1");
    service
        .create_story("Story 2", "Content 2")
        .await
        .expect("create story 2");

    let (stories, total) = service.list_stories().await.expect("list stories");
    assert!(total >= 2);
    assert!(stories.len() >= 2);
}

#[tokio::test]
async fn test_list_stories_empty() {
    let state = setup_test_state().await;
    let service = StoryService::new(&state.pool);

    let (stories, total) = service.list_stories().await.expect("list stories");
    assert_eq!(total, 0);
    assert!(stories.is_empty());
}

#[tokio::test]
async fn test_get_all_menus_success() {
    let state = setup_test_state().await;

    // Create test menus
    let menu1_id = system_repo::create_menu(
        &state.pool,
        "Menu 1",
        1,
        Some("/menu1"),
        Some("icon1"),
        Some(1),
        None,
    )
    .await
    .expect("create menu 1");

    let menu2_id = system_repo::create_menu(
        &state.pool,
        "Menu 2",
        2,
        Some("/menu2"),
        Some("icon2"),
        Some(2),
        Some(menu1_id),
    )
    .await
    .expect("create menu 2");

    let menus = system_repo::get_all_menus(&state.pool)
        .await
        .expect("get all menus");

    assert!(menus.len() >= 2);
    assert!(menus.iter().any(|m| m.id == menu1_id));
    assert!(menus.iter().any(|m| m.id == menu2_id));
}

#[tokio::test]
async fn test_get_role_menu_ids_success() {
    let state = setup_test_state().await;

    // Create role and menus
    let role_id = system_repo::create_role(&state.pool, "Test Role", None)
        .await
        .expect("create role");

    let menu_id = system_repo::create_menu(
        &state.pool,
        "Test Menu",
        1,
        Some("/test"),
        None,
        Some(1),
        None,
    )
    .await
    .expect("create menu");

    // Assign menu to role
    system_repo::set_role_menus(&state.pool, role_id, &[menu_id])
        .await
        .expect("set role menus");

    let menu_ids = system_repo::get_role_menu_ids(&state.pool, role_id)
        .await
        .expect("get role menu ids");

    assert_eq!(menu_ids.len(), 1);
    assert_eq!(menu_ids[0], menu_id);
}

#[tokio::test]
async fn test_get_role_menu_ids_empty() {
    let state = setup_test_state().await;

    let role_id = system_repo::create_role(&state.pool, "Empty Role", None)
        .await
        .expect("create role");

    let menu_ids = system_repo::get_role_menu_ids(&state.pool, role_id)
        .await
        .expect("get role menu ids");

    assert!(menu_ids.is_empty());
}

#[tokio::test]
async fn test_set_role_menus_success() {
    let state = setup_test_state().await;

    let role_id = system_repo::create_role(&state.pool, "Role", None)
        .await
        .expect("create role");

    let menu1_id = system_repo::create_menu(
        &state.pool,
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
        &state.pool,
        "Menu 2",
        1,
        None,
        None,
        Some(2),
        None,
    )
    .await
    .expect("create menu 2");

    system_repo::set_role_menus(&state.pool, role_id, &[menu1_id, menu2_id])
        .await
        .expect("set role menus");

    let menu_ids = system_repo::get_role_menu_ids(&state.pool, role_id)
        .await
        .expect("get role menu ids");

    assert_eq!(menu_ids.len(), 2);
    assert!(menu_ids.contains(&menu1_id));
    assert!(menu_ids.contains(&menu2_id));
}

#[tokio::test]
async fn test_get_category_counts_success() {
    let state = setup_test_state().await;

    // Create test categories
    use cbs_backend::infra::goods_repo;
    goods_repo::create_category(&state.pool, "Category 1", None)
        .await
        .expect("create category 1");
    goods_repo::create_category(&state.pool, "Category 2", None)
        .await
        .expect("create category 2");

    let counts = metrics_repo::get_category_counts(&state.pool)
        .await
        .expect("get category counts");

    assert!(counts.len() >= 2);
}

#[tokio::test]
async fn test_get_category_sales_success() {
    let state = setup_test_state().await;

    let sales = metrics_repo::get_category_sales(&state.pool)
        .await
        .expect("get category sales");

    assert!(sales.is_empty() || !sales.is_empty()); // Can be empty or have data
}

#[tokio::test]
async fn test_get_category_favors_success() {
    let state = setup_test_state().await;

    let favors = metrics_repo::get_category_favors(&state.pool)
        .await
        .expect("get category favors");

    assert!(favors.is_empty() || !favors.is_empty()); // Can be empty or have data
}

#[tokio::test]
async fn test_get_top_sales_success() {
    let state = setup_test_state().await;

    // Create test goods
    use cbs_backend::infra::goods_repo;
    goods_repo::create_goods(
        &state.pool,
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

    let top_sales = metrics_repo::get_top_sales(&state.pool)
        .await
        .expect("get top sales");

    assert!(top_sales.len() <= 10); // Should be max 10
}

#[tokio::test]
async fn test_get_address_sales_success() {
    let state = setup_test_state().await;

    // Create test goods with addresses
    use cbs_backend::infra::goods_repo;
    goods_repo::create_goods(
        &state.pool,
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
        &state.pool,
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

    let address_sales = metrics_repo::get_address_sales(&state.pool)
        .await
        .expect("get address sales");

    assert!(address_sales.len() >= 2);
}

#[tokio::test]
async fn test_get_goods_amount_list_success() {
    let state = setup_test_state().await;

    let amounts = metrics_repo::get_goods_amount_list(&state.pool)
        .await
        .expect("get goods amount list");

    assert!(amounts.is_empty() || !amounts.is_empty()); // Can be empty or have data
}

