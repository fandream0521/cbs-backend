use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::Deserialize;

use crate::{
    api::ApiResponse,
    domain::story::StoryListRequest,
    infra::state::AppState,
    services::story_service::StoryService,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStoryBody {
    pub title: String,
    pub content: String,
}

fn service(state: &AppState) -> StoryService<'_> {
    StoryService::new(&state.pool)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/story", post(create_story))
        .route("/story/list", post(list_stories))
}

async fn create_story(
    State(state): State<AppState>,
    Json(body): Json<CreateStoryBody>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    if body.title.trim().is_empty() || body.content.trim().is_empty() {
        return Err(crate::domain::error::DomainError::InvalidInput(
            "title and content are required".into(),
        ));
    }

    let id = service(&state)
        .create_story(&body.title, &body.content)
        .await?;
    Ok(Json(ApiResponse::success(serde_json::json!({ "id": id }))))
}

async fn list_stories(
    State(state): State<AppState>,
    Json(_body): Json<StoryListRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, crate::domain::error::DomainError> {
    let (list, total) = service(&state).list_stories().await?;
    Ok(Json(ApiResponse::success(
        serde_json::json!({ "list": list, "totalCount": total }),
    )))
}

