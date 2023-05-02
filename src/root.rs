//! Root routing

use crate::{api, database::Database, server_state::ServerState};
use axum::Router;
use tower_http::trace::TraceLayer;

/// Creates a router for the root node of the server.
pub fn create_router<D: Database + 'static>() -> Router<ServerState<D>> {
    Router::new()
        .nest("/api", api::create_router())
        .layer(TraceLayer::new_for_http())
}
