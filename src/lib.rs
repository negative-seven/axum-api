pub mod api;
pub mod database;
pub mod root;
mod server_state;
pub mod token;

pub use server_state::ServerState;

use std::{error::Error, net::SocketAddr};

/// Runs the server.
///
/// Binds to the specified address and utilizes the provided [`ServerState`]
/// object for storing state.
///
/// # Errors
///
/// Unhandled server errors from the library level get bubbled up as errors
/// returned by this function.
///
/// # Example
///
/// ```no_run
/// use axum_api::{database::SimpleMemoryDatabase, run_server, token::TokenManager, ServerState};
/// use std::fs;
/// use std::time::Duration;
/// use tokio::task;
///
/// let database = SimpleMemoryDatabase::new();
/// let token_manager = TokenManager::new(
///     Duration::from_secs(10 * 60),
///     Duration::from_secs(20),
///     jsonwebtoken::Algorithm::RS512,
///     fs::read_to_string("secret").expect("could not read secret from file"),
/// );
/// task::spawn(async {
///     run_server(
///         &"172.16.0.1:3000".parse().expect("could not parse address"),
///         ServerState::new(database, token_manager),
///     )
///     .await
///     .expect("encountered server error");
/// });
/// ```
pub async fn run_server<D: database::Database + 'static>(
    address: &SocketAddr,
    state: ServerState<D>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    axum::Server::bind(address)
        .serve(
            root::create_router::<D>()
                .with_state(state)
                .into_make_service(),
        )
        .await?;

    Ok(())
}
