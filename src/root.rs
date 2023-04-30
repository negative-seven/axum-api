use crate::api;
use axum::Router;

pub fn create_router() -> Router {
    Router::new().nest("/api", api::create_router())
}
