use crate::database::Database;

/// The internal state of the server.
///
/// Stores data, handles, caches, etc. that ought to persist for the lifetime of
/// the server.
#[derive(Clone)]
pub struct ServerState<D: Database> {
    /// A database access object which can be utilized by the server.
    pub database: D,
}
