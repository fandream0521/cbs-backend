use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub create_at: Option<String>,
    pub update_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryListRequest {
    // Empty body for list request
}

