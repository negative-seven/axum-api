//! Simple, extensible REST API service utilizing the [axum](axum) framework.
//!
//! This crate provides a [`Router`](axum::Router) for API endpoints. A `main` function is also
//! given, which allows for easily running a simple server that reroutes all API
//! endpoints under `/api`.
mod api;
pub mod database;
mod server_state;
pub mod token;

pub use api::create_api_router;
pub use server_state::ServerState;
