use crate::{api, database::Database, server_state::ServerState};
use axum::Router;
use tower_http::trace::TraceLayer;

pub fn create_router<D: Database + 'static>() -> Router<ServerState<D>> {
    Router::new()
        .nest("/api", api::create_router())
        .layer(TraceLayer::new_for_http())
}
