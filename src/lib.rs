pub mod api;
pub mod database;
pub mod root;
mod server_state;
mod token;

pub use server_state::ServerState;
pub use token::TokenPayload;

use std::{error::Error, net::SocketAddr};

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
