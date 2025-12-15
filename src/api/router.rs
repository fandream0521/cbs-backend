use axum::{Router, middleware::from_fn_with_state, routing::get, routing::post};
use std::sync::Arc;

use crate::{
    api::auth::{login, test_token},
    api::goods,
    api::menu_tree,
    api::metrics,
    api::story,
    api::system,
    auth::{AuthState, guard_bearer},
    infra::{cors::cors_layer, logging::trace_layer, state::AppState},
};

pub fn build_router(state: AppState) -> Router {
    let auth_state = Arc::new(AuthState {
        allow_any_token: true, // TODO: replace with real token verification.
    });

    Router::new()
        .route("/health", get(health))
        .route("/login", post(login))
        .route("/test", get(test_token))
        .nest("/", system::routes())
        .nest("/", goods::routes())
        .nest("/", story::routes())
        .nest("/", menu_tree::routes())
        .nest("/", metrics::routes())
        .layer(from_fn_with_state(auth_state, guard_bearer))
        .layer(trace_layer())
        .layer(cors_layer())
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
