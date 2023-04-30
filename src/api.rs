use crate::server_state::ServerState;
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

pub fn create_router() -> Router<Arc<Mutex<ServerState>>> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}

#[allow(clippy::unused_async)]
async fn login(
    State(state): State<Arc<Mutex<ServerState>>>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let mut state = state.lock().unwrap();
    let logged_in_old = state.logged_in;
    state.logged_in = true;
    Json(json!({
        "login_message": body["message"],
        "login_state": {
            "old": logged_in_old,
            "new": state.logged_in,
        }
    }))
}

#[allow(clippy::unused_async)]
async fn logout(
    State(state): State<Arc<Mutex<ServerState>>>,
    Json(body): Json<Value>,
) -> Json<Value> {
    let mut state = state.lock().unwrap();
    let logged_in_old = state.logged_in;
    state.logged_in = false;
    Json(json!({
        "logout_message": body["message"],
        "login_state": {
            "old": logged_in_old,
            "new": state.logged_in,
        }
    }))
}
