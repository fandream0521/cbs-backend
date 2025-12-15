use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Goods {
    pub id: i64,
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
    pub create_at: Option<String>,
    pub update_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub create_at: Option<String>,
    pub update_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsPagination {
    pub offset: i64,
    pub size: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryPagination {
    pub offset: i64,
    pub size: i64,
    #[serde(default)]
    pub name: Option<String>,
}

