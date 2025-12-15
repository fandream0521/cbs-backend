use anyhow::Context;
use sqlx::ConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub const DB_PATH: &str = "./.cms_backend/cms_backend.db";

pub async fn connect_pool() -> anyhow::Result<SqlitePool> {
    ensure_db_dir()?;

    let options = SqliteConnectOptions::from_str(DB_PATH)?
        .create_if_missing(true)
        .log_statements(log::LevelFilter::Info);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .context("connect sqlite pool")
}

fn ensure_db_dir() -> anyhow::Result<()> {
    if let Some(parent) = Path::new(DB_PATH).parent() {
        fs::create_dir_all(parent).context("create db directory")?;
    }
    Ok(())
}

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("run migrations")
}
