use axum_api::{database::ScyllaDbSession, run_server, ServerState, TokenManager};
use std::{error::Error, time::Duration};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // TODO: make these parameters configurable
    run_server(
        &"0.0.0.0:3000".parse()?,
        ServerState::new(
            ScyllaDbSession::new(&["localhost:9042"]).await?,
            TokenManager::new(
                Duration::from_secs(30 * 60),
                Duration::from_secs(60),
                jsonwebtoken::Algorithm::HS256,
                "secret".into(),
            ),
        ),
    )
    .await
}
