use crate::{database::Database, token::TokenManager};
use std::sync::Arc;

/// The internal state of the server.
///
/// Stores data, handles, caches, etc. that ought to persist for the lifetime of
/// the server.
#[derive(Clone)]
pub struct ServerState<D: Database> {
    /// A database access object which can be utilized by the server.
    database: D,

    /// Token manager for API JWTs.
    token_manager: Arc<TokenManager>,
}

impl<D: Database> ServerState<D> {
    // Creates a server state object.
    pub fn new(database: D, token_manager: TokenManager) -> Self {
        Self {
            database,
            token_manager: Arc::new(token_manager),
        }
    }

    pub fn database(&self) -> &D {
        &self.database
    }

    pub fn token_manager(&self) -> Arc<TokenManager> {
        Arc::clone(&self.token_manager)
    }
}
