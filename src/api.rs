//! API routing.

use crate::{
    database::{self, Database},
    server_state::ServerState,
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

/// Creates a router for API endpoints.
pub fn create_api_router<D: Database + 'static>() -> Router<ServerState<D>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/token", get(get_token))
}

/// Handler for user registration.
async fn register<D: Database>(
    State(state): State<ServerState<D>>,
    Json(user): Json<database::User>,
) -> impl IntoResponse {
    if !state.database().try_add_user(user).await {
        // TODO: assumed to be an e-mail conflict, but other kinds of errors are
        // possible in the future
        info!("could not add new user to database due to email conflict with existing user");
        return StatusCode::CONFLICT;
    }

    StatusCode::OK
}

/// Handler for generating an API token for a user.
async fn login<D: Database>(
    State(state): State<ServerState<D>>,
    Json(user): Json<database::User>,
) -> impl IntoResponse {
    if !state.database().validate_user(&user).await {
        info!("invalid credentials provided during login");
        return (StatusCode::UNAUTHORIZED, "").into_response();
    }

    let token = if let Ok(result) = state.token_manager().new_token(&user) {
        result
    } else {
        warn!("could not create token for user");
        return (StatusCode::INTERNAL_SERVER_ERROR, "").into_response();
    };

    (StatusCode::OK, Json(json!({ "token": token }))).into_response()
}

/// Handler for checking the validity of a token.
#[allow(clippy::unused_async)]
async fn get_token<D: Database>(
    State(state): State<ServerState<D>>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let token = authorization.token();
    let token_payload = state
        .token_manager()
        .decode_and_validate_token(token.into());

    let mut response = json!({"token": token, "valid": token_payload.is_ok()});
    if let Ok(payload) = token_payload {
        response["user_email"] = payload.user_email.into();
    };

    Json(response)
}
