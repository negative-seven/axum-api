pub mod api;
pub mod database;
pub mod root;
mod server_state;
mod token;

pub use server_state::ServerState;
pub use token::TokenPayload;

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
/// use axum_api::{database::SimpleMemoryDatabase, run_server, ServerState};
/// use tokio::task;
///
/// task::spawn(async {
///     run_server(
///         &"172.16.0.1:3000".parse().expect("could not parse address"),
///         ServerState::new(SimpleMemoryDatabase::new().expect("could not create database"))
///             .await
///             .expect("could not create server state"),
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
