#[allow(unused_imports)]
use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    api::ApiResponse,
    domain::goods::{Category, CategoryPagination, Goods, GoodsPagination},
    infra::state::AppState,
    services::goods_service::GoodsService,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGoodsBody {
    pub name: String,
    pub old_price: Option<f64>,
    pub new_price: Option<f64>,
    pub desc: Option<String>,
    pub status: Option<i32>,
    pub img_url: Option<String>,
    pub inventory_count: Option<i64>,
    pub sale_count: Option<i64>,
    pub favor_count: Option<i64>,
    pub address: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGoodsBody {
    pub name: Option<String>,
    pub old_price: Option<f64>,
    pub new_price: Option<f64>,
    pub desc: Option<String>,
    pub status: Option<i32>,
    pub img_url: Option<String>,
    pub inventory_count: Option<i64>,
    pub sale_count: Option<i64>,
    pub favor_count: Option<i64>,
    pub address: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryBody {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategoryBody {
    pub name: Option<String>,
    pub parent_id: Option<i64>,
}

fn service(state: &AppState) -> GoodsService<'_> {
    GoodsService::new(&state.pool)
}

fn validate_goods_pagination(p: &GoodsPagination) -> Result<(), crate::domain::error::DomainError> {
    if p.size <= 0 || p.size > 100 {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "size must be between 1 and 100".into(),
        ));
    }
    if p.offset < 0 {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "offset must be >= 0".into(),
        ));
    }
    Ok(())
}

fn validate_category_pagination(
    p: &CategoryPagination,
) -> Result<(), crate::domain::error::DomainError> {
    if p.size <= 0 || p.size > 100 {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "size must be between 1 and 100".into(),
        ));
    }
    if p.offset < 0 {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "offset must be >= 0".into(),
        ));
    }
    Ok(())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        // Goods
        .route("/goods", patch(create_goods))
        .route(
            "/goods/:id",
            patch(update_goods).delete(delete_goods).get(get_goods),
        )
        .route("/goods/list", post(list_goods))
        // Categories
        .route("/category", post(create_category))
        .route(
            "/category/:id",
            patch(update_category).delete(delete_category).get(get_category),
        )
        .route("/category/list", post(list_categories))
}

async fn create_goods(
    State(state): State<AppState>,
    Json(body): Json<CreateGoodsBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    if body.name.trim().is_empty() {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "goods name is required".into(),
        ));
    }

    let id = service(&state)
        .create_goods(
            &body.name,
            body.old_price,
            body.new_price,
            body.desc.as_deref(),
            body.status,
            body.img_url.as_deref(),
            body.inventory_count,
            body.sale_count,
            body.favor_count,
            body.address.as_deref(),
        )
        .await?;
    Ok(Json(ApiResponse::success(serde_json::json!({ "id": id }))))
}

async fn update_goods(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateGoodsBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state)
        .update_goods(
            id,
            body.name.as_deref(),
            body.old_price,
            body.new_price,
            body.desc.as_deref(),
            body.status,
            body.img_url.as_deref(),
            body.inventory_count,
            body.sale_count,
            body.favor_count,
            body.address.as_deref(),
        )
        .await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "updated": true }),
    )))
}

async fn delete_goods(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state).delete_goods(id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "deleted": true }),
    )))
}

async fn get_goods(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Goods>>, crate::domain::error::DomainError> {
    let goods = service(&state).get_goods(id).await?;
    Ok(Json(ApiResponse::success(goods)))
}

async fn list_goods(
    State(state): State<AppState>,
    Json(pagination): Json<GoodsPagination>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    validate_goods_pagination(&pagination)?;
    let (list, total) = service(&state).list_goods(&pagination).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "list": list, "totalCount": total }),
    )))
}

async fn create_category(
    State(state): State<AppState>,
    Json(body): Json<CreateCategoryBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    if body.name.trim().is_empty() {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "category name is required".into(),
        ));
    }
    let id = service(&state)
        .create_category(&body.name, None)
        .await?;
    Ok(Json(ApiResponse::success(serde_json::json!({ "id": id }))))
}

async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateCategoryBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state)
        .update_category(id, body.name.as_deref(), body.parent_id)
        .await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "updated": true }),
    )))
}

async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    service(&state).delete_category(id).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "deleted": true }),
    )))
}

async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Category>>, crate::domain::error::DomainError> {
    let category = service(&state).get_category(id).await?;
    Ok(Json(ApiResponse::success(category)))
}

async fn list_categories(
    State(state): State<AppState>,
    Json(pagination): Json<CategoryPagination>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    validate_category_pagination(&pagination)?;
    let (list, total) = service(&state).list_categories(&pagination).await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "list": list, "totalCount": total }),
    )))
}

