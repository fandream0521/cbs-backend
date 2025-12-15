use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use tempfile::NamedTempFile;

pub async fn test_pool() -> SqlitePool {
    let file = NamedTempFile::new().expect("temp db file");
    let path = file.path().to_string_lossy().to_string();

    SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!("sqlite://{}", path))
        .await
        .expect("connect temp sqlite")
}
