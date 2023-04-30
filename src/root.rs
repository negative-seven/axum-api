use std::sync::{Arc, Mutex};

use crate::{api, server_state::ServerState};
use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .nest("/api", api::create_router())
        .with_state(Arc::new(Mutex::new(ServerState::new())))
}
