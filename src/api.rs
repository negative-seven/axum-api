use crate::{database, server_state::ServerState, token};
use axum::{
    extract::State,
    headers::Cookie,
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router, TypedHeader,
};
use serde_json::json;
use tracing::{info, warn};

pub fn create_router() -> Router<ServerState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/token", get(get_token))
}

#[allow(clippy::unused_async)]
async fn register(
    State(state): State<ServerState>,
    Json(body): Json<serde_json::Value>, // TODO: database::User directly?
) -> impl IntoResponse {
    let user: database::User = if let Ok(u) = serde_json::value::from_value(body) {
        u
    } else {
        warn!("could not convert Value to User");
        return StatusCode::UNPROCESSABLE_ENTITY;
    };

    if !state.database.try_add_user(user) {
        // TODO: assumed to be an e-mail conflict, but other kinds of errors are
        // possible in the future
        info!("could not add new user to database due to email conflict with existing user");
        return StatusCode::CONFLICT;
    }

    StatusCode::OK
}

#[allow(clippy::unused_async)]
async fn login(
    State(state): State<ServerState>,
    Json(user): Json<database::User>,
) -> impl IntoResponse {
    if !state.database.validate_user(&user) {
        info!("invalid credentials provided during login");
        return (StatusCode::UNAUTHORIZED, HeaderMap::default());
    }

    let token = if let Ok(result) = token::create() {
        result
    } else {
        warn!("could not create token for user");
        return (StatusCode::INTERNAL_SERVER_ERROR, HeaderMap::default());
    };

    let mut response_headers = HeaderMap::new();
    response_headers.append(
        SET_COOKIE,
        HeaderValue::from_str(&format!("api_token={token}; Secure; HttpOnly"))
            .expect("failed to convert cookie String to HeaderValue"),
    );
    (StatusCode::OK, response_headers)
}

#[allow(clippy::unused_async)]
async fn get_token(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {
    match cookie.get("api_token") {
        Some(token) => (
            StatusCode::OK,
            Json(json!({
                "token": token,
                "valid": token::is_valid(token)
            })),
        ),
        None => (StatusCode::UNAUTHORIZED, Json::default()),
    }
}
