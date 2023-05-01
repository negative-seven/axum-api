mod api;
mod database;
mod root;
mod server_state;
mod token;

use std::error::Error;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(root::create_router().into_make_service())
        .await?;

    Ok(())
}
