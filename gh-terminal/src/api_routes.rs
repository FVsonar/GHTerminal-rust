use std::sync::Arc;
use axum::routing::get;
use axum::{Extension, Router};

use crate::assets;
use crate::state::AppState;
use crate::ws;

pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(assets::index))
        .route("/ws", get(ws::ws_handler))
        .fallback(get(assets::serve_static))
        .layer(Extension(state))
}
