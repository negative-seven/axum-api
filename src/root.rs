use crate::{api, server_state::ServerState};
use axum::Router;
use tower_http::trace::TraceLayer;

pub fn create_router() -> Router {
    Router::new()
        .nest("/api", api::create_router())
        .layer(TraceLayer::new_for_http())
        .with_state(ServerState::new())
}
