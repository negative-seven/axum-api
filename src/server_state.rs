use crate::database::Database;
use std::error::Error;
use tracing::debug;

#[derive(Clone)]
pub struct ServerState<D: Database> {
    pub database: D,
}

impl<D: Database> ServerState<D> {
    pub async fn new(database: D) -> Result<Self, Box<dyn Error + Send + Sync>> {
        debug!("creating new ServerState");

        Ok(Self { database })
    }
}
