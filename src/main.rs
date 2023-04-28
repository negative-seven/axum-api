use axum::{routing, Router};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/a", routing::get(|| async { "endpoint a" }))
        .route("/b", routing::get(|| async { "endpoint b" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
