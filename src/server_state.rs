use crate::database::SimpleMemoryDatabase;

#[derive(Clone)]
pub struct ServerState {
    pub database: SimpleMemoryDatabase,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            database: SimpleMemoryDatabase::new(),
        }
    }
}
