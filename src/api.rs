use crate::{server_state::ServerState, token};
use axum::{
    headers::Cookie,
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router, TypedHeader,
};
use serde_json::json;
use std::sync::{Arc, Mutex};

pub fn create_router() -> Router<Arc<Mutex<ServerState>>> {
    Router::new()
        .route("/login", post(login))
        .route("/token", get(get_token))
}

#[allow(clippy::unused_async)]
async fn login() -> impl IntoResponse {
    let token = token::create();

    let mut response_headers = HeaderMap::new();
    response_headers.append(
        SET_COOKIE,
        HeaderValue::from_str(&format!("api_token={token}; Secure; HttpOnly")).unwrap(),
    );
    response_headers
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
