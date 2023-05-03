use axum_api::{database::SimpleMemoryDatabase, token::TokenManager, ServerState};
use reqwest::StatusCode;
use serde_json::{Map, Value};
use std::{error::Error, future::Future, time::Duration};
use tokio::task;

const ADDRESS: &str = "127.0.0.1:29200";

pub struct Response {
    pub status_code: StatusCode,
    pub body: Option<Map<String, Value>>,
}

pub async fn with_server(
    future: impl Future<Output = Result<(), Box<dyn Error>>>,
) -> Result<(), Box<dyn Error>> {
    let server_task = task::spawn(async {
        axum_api::run_server(
            &ADDRESS.parse().unwrap(),
            ServerState::new(
                SimpleMemoryDatabase::new(),
                TokenManager::new(
                    Duration::from_secs(10u64.pow(10)),
                    Duration::from_secs(10u64.pow(10)),
                    jsonwebtoken::Algorithm::HS256,
                    "secret".into(),
                ),
            ),
        )
        .await
    });

    let return_value = future.await;
    server_task.abort();
    return_value
}

pub async fn post(endpoint: impl AsRef<str>, json: Value) -> Response {
    let response = reqwest::Client::new()
        .post(format!("http://{ADDRESS}/api/{}", endpoint.as_ref()))
        .json(&json)
        .send()
        .await
        .unwrap();
    drop(json);

    Response {
        status_code: response.status(),
        body: response.json::<Map<String, Value>>().await.ok(),
    }
}
