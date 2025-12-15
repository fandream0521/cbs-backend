mod api;
mod auth;
mod domain;
mod infra;
mod services;

use anyhow::Context;
use api::router::build_router;
use infra::db::{connect_pool, run_migrations};
use infra::state::AppState;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    infra::logging::init_tracing()?;

    let pool = connect_pool().await?;
    run_migrations(&pool).await?;
    let app_state = AppState {
        pool: std::sync::Arc::new(pool),
    };

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .context("bind server socket")?;
    info!("listening on {}", listener.local_addr()?);

    let app = build_router(app_state);
    axum::serve(listener, app.into_make_service())
        .await
        .context("start http server")
}
