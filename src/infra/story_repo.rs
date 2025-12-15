use crate::domain::story::Story;
use anyhow::Context;
use sqlx::{Row, SqlitePool};
use tracing::instrument;

#[instrument(skip(pool, title, content))]
pub async fn create_story(
    pool: &SqlitePool,
    title: &str,
    content: &str,
) -> anyhow::Result<i64> {
    let rec = sqlx::query(
        r#"
        INSERT INTO stories (title, content)
        VALUES (?1, ?2)
        RETURNING id
        "#,
    )
    .bind(title)
    .bind(content)
    .fetch_one(pool)
    .await
    .context("insert story")?;
    Ok(rec.get("id"))
}

#[instrument(skip(pool))]
pub async fn list_stories(pool: &SqlitePool) -> anyhow::Result<(Vec<Story>, i64)> {
    let list = sqlx::query_as::<_, Story>(
        r#"
        SELECT id, title, content, create_at, update_at
        FROM stories
        ORDER BY create_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .context("list stories")?;

    let total: i64 = sqlx::query("SELECT COUNT(*) as count FROM stories")
        .fetch_one(pool)
        .await
        .context("count stories")?
        .get("count");

    Ok((list, total))
}

