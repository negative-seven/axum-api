use axum::{routing::post, Json, Router};
use serde_json::{json, Value};

pub fn create_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
}

#[allow(clippy::unused_async)]
async fn login(Json(body): Json<Value>) -> Json<Value> {
    Json(json!({"login_message": body["message"]}))
}

#[allow(clippy::unused_async)]
async fn logout(Json(body): Json<Value>) -> Json<Value> {
    Json(json!({"logout_message": body["message"]}))
}
