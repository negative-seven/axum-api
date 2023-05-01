use crate::database::Database;
use std::error::Error;
use tracing::debug;

#[derive(Clone)]
pub struct ServerState<D: Database> {
    pub database: D,
}

impl<D: Database> ServerState<D> {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        debug!("creating new ServerState");

        Ok(Self {
            database: D::new().await?,
        })
    }
}
