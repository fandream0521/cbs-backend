use crate::{
    domain::error::DomainError,
    domain::story::Story,
    infra::story_repo,
};
use sqlx::SqlitePool;

pub struct StoryService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> StoryService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_story(&self, title: &str, content: &str) -> Result<i64, DomainError> {
        story_repo::create_story(self.pool, title, content)
            .await
            .map_err(DomainError::Internal)
    }

    pub async fn list_stories(&self) -> Result<(Vec<Story>, i64), DomainError> {
        story_repo::list_stories(self.pool)
            .await
            .map_err(DomainError::Internal)
    }
}

