use axum_api::{database::ScyllaDbSession, run_server, ServerState};
use std::error::Error;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // TODO: make hosts configurable
    run_server(
        &"0.0.0.0:3000".parse()?,
        ServerState::new(ScyllaDbSession::new(&["localhost:9042"]).await?).await?,
    )
    .await
}
