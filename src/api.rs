use crate::{
    database::{self, Database},
    server_state::ServerState,
    token::TokenPayload,
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, TypedHeader,
};
use serde_json::json;
use tracing::{info, warn};

pub fn create_router<D: Database + 'static>() -> Router<ServerState<D>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/token", get(get_token))
}

async fn register<D: Database>(
    State(state): State<ServerState<D>>,
    Json(user): Json<database::User>,
) -> impl IntoResponse {
    if !state.database.try_add_user(user).await {
        // TODO: assumed to be an e-mail conflict, but other kinds of errors are
        // possible in the future
        info!("could not add new user to database due to email conflict with existing user");
        return StatusCode::CONFLICT;
    }

    StatusCode::OK
}

async fn login<D: Database>(
    State(state): State<ServerState<D>>,
    Json(user): Json<database::User>,
) -> impl IntoResponse {
    if !state.database.validate_user(&user).await {
        info!("invalid credentials provided during login");
        return (StatusCode::UNAUTHORIZED, Json(json!({})));
    }

    let token = if let Ok(result) = TokenPayload::new().encode() {
        result
    } else {
        warn!("could not create token for user");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})));
    };

    (StatusCode::OK, Json(json!({ "token": token })))
}

#[allow(clippy::unused_async)]
async fn get_token(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let token = authorization.token();
    Json(json!({
        "token": token,
        "valid": TokenPayload::decode(token).is_ok()
    }))
}
