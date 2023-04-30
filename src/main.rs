use std::error::Error;

mod api;
mod root;
mod server_state;
mod token;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(root::create_router().into_make_service())
        .await?;

    Ok(())
}
