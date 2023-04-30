#[derive(Clone)]
pub struct ServerState {
    pub logged_in: bool, // TODO: this is global across the server, not unique to each session
}

impl ServerState {
    pub fn new() -> Self {
        Self { logged_in: false }
    }
}
