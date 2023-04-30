mod api;
mod root;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(root::create_router().into_make_service())
        .await
        .unwrap();
}