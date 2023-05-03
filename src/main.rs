use axum::Router;
use axum_api::{create_api_router, database::ScyllaDbSession, token::TokenManager, ServerState};
use std::{error::Error, time::Duration};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

/// Runs a simple server which merely reroutes requests to /api to the API
/// router.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // TODO: make all these parameters configurable
    let state = ServerState::new(
        ScyllaDbSession::new(&["localhost:9042"]).await?,
        TokenManager::new(
            Duration::from_secs(30 * 60),
            Duration::from_secs(60),
            jsonwebtoken::Algorithm::HS256,
            "secret".into(),
        ),
    );
    let root_router = Router::new()
        .nest("/api", create_api_router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(root_router.into_make_service())
        .await?;

    Ok(())
}
