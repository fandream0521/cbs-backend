use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub realname: String,
    pub cellphone: Option<String>,
    pub enable: i32,
    pub department_id: Option<i64>,
    pub role_id: Option<i64>,
    pub create_at: Option<String>,
    pub update_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Department {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub leader: Option<String>,
    pub create_at: Option<String>,
    pub update_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub intro: Option<String>,
    pub create_at: Option<String>,
    pub update_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub id: i64,
    pub name: String,
    pub r#type: i32,
    pub url: Option<String>,
    pub icon: Option<String>,
    pub sort: Option<i32>,
    pub parent_id: Option<i64>,
    pub create_at: Option<String>,
    pub update_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub offset: i64,
    pub size: i64,
    #[serde(default)]
    pub name: Option<String>,
}
