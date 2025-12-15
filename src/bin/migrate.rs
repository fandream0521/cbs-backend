use anyhow::Context;
use cbs_backend::infra::db::{connect_pool, run_migrations};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = connect_pool().await.context("connect pool")?;
    run_migrations(&pool).await.context("run migrations")?;
    Ok(())
}
