use axum::Router;
use axum_api::{create_api_router, database::ScyllaDbSession, token::TokenManager, ServerState};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, str::FromStr, time::Duration};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
struct Arguments {
    /// Config file path
    #[arg(short, long, default_value = "resources/config.json")]
    config_file: String,

    /// Generate a default config file and exit. This will overwrite the file if
    /// it already exists.
    #[arg(short, long)]
    generate_config: bool,
}

/// Server config
#[derive(Serialize, Deserialize)]
struct Config {
    /// Host for the server to bind to.
    server_host: String,

    /// Hosts which the ScyllaDB instance is listening on.
    database_hosts: Vec<String>,

    /// Lifetime of an API token in seconds
    lifetime: u64,

    /// Leeway for lifetime checks of API tokens in seconds
    lifetime_leeway: u64,

    /// Algorithm used to sign JSON web tokens. Must be supported by the
    /// `jsonwebtoken` crate.
    signing_algorithm: String,

    /// Path to a file containing the secret used for JSON web token generation.
    secret_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_host: "127.0.0.1:3000".to_string(),
            database_hosts: vec!["127.0.0.1:9042".to_string()],
            lifetime: 600,
            lifetime_leeway: 30,
            signing_algorithm: "HS256".to_string(),
            secret_path: "resources/secret".to_string(),
        }
    }
}

/// Runs a simple server which merely reroutes requests to /api to the API
/// router.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let arguments = Arguments::parse();

    if arguments.generate_config {
        fs::write(
            &arguments.config_file,
            serde_json::to_string_pretty(&Config::default())?,
        )?;

        return Ok(());
    }

    let config = serde_json::from_slice::<Config>(&fs::read(&arguments.config_file)?)?;

    let state = ServerState::new(
        ScyllaDbSession::new(&config.database_hosts).await?,
        TokenManager::new(
            Duration::from_secs(config.lifetime),
            Duration::from_secs(config.lifetime_leeway),
            jsonwebtoken::Algorithm::from_str(config.signing_algorithm.as_str())?,
            fs::read_to_string(&config.secret_path)?,
        ),
    );
    let root_router = Router::new()
        .nest("/api", create_api_router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    axum::Server::bind(&config.server_host.parse()?)
        .serve(root_router.into_make_service())
        .await?;

    Ok(())
}
