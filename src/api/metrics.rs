use axum::{
    extract::State,
    routing::get,
    Json, Router,
};

use crate::{
    api::ApiResponse,
    infra::{metrics_repo, state::AppState},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/goods/category/count", get(get_category_counts))
        .route("/goods/category/sale", get(get_category_sales))
        .route("/goods/category/favor", get(get_category_favors))
        .route("/goods/sale/top10", get(get_top_sales))
        .route("/goods/address/sale", get(get_address_sales))
        .route("/goods/amount/list", get(get_goods_amount_list))
}

async fn get_category_counts(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<metrics_repo::CategoryCount>>>, crate::domain::error::DomainError> {
    let counts = metrics_repo::get_category_counts(&state.pool)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    Ok(Json(ApiResponse::success(counts)))
}

async fn get_category_sales(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<metrics_repo::CategorySale>>>, crate::domain::error::DomainError> {
    let sales = metrics_repo::get_category_sales(&state.pool)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    Ok(Json(ApiResponse::success(sales)))
}

async fn get_category_favors(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<metrics_repo::CategoryFavor>>>, crate::domain::error::DomainError> {
    let favors = metrics_repo::get_category_favors(&state.pool)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    Ok(Json(ApiResponse::success(favors)))
}

async fn get_top_sales(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<metrics_repo::TopSale>>>, crate::domain::error::DomainError> {
    let top_sales = metrics_repo::get_top_sales(&state.pool)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    Ok(Json(ApiResponse::success(top_sales)))
}

async fn get_address_sales(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<metrics_repo::AddressSale>>>, crate::domain::error::DomainError> {
    let address_sales = metrics_repo::get_address_sales(&state.pool)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    Ok(Json(ApiResponse::success(address_sales)))
}

async fn get_goods_amount_list(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<metrics_repo::CategoryCount>>>, crate::domain::error::DomainError> {
    let amounts = metrics_repo::get_goods_amount_list(&state.pool)
        .await
        .map_err(crate::domain::error::DomainError::Internal)?;
    Ok(Json(ApiResponse::success(amounts)))
}

