mod common;

use common::{post, with_server};
use reqwest::StatusCode;
use serde_json::json;
use serial_test::serial;
use std::error::Error;

#[tokio::test]
#[serial]
async fn register_multiple_successfully() -> Result<(), Box<dyn Error>> {
    with_server(async {
        for (email, password) in [
            ("email@addre.ss", "2kj7g435yewrktjh"),
            ("email2@addre.ss", "sdkh4k2qj43lw"),
            ("email3@addre.ss", "125c3kj4hk3huwy"),
            ("email@add.ress", "12jk67h354jl21"),
            ("someone@gmail.com", "j2h4kj623467385"),
        ] {
            let response = post("register", json!({"email": email, "password": password})).await;
            assert_eq!(response.status_code, StatusCode::OK);
        }

        Ok(())
    })
    .await
}

#[tokio::test]
#[serial]
async fn register_with_duplicate_email() -> Result<(), Box<dyn Error>> {
    with_server(async {
        post(
            "register",
            json!({"email": "dupl@ica.te", "password": "pw"}),
        )
        .await;

        for (email, password) in [
            ("dupl@ica.te", "kjh3k4734g6h12j5"),
            ("dupl@ica.te", "khk45h8kjl68h2346"),
            ("dupl@ica.te", "lkjhlkjh7845j2371"),
            ("dupl@ica.te", "kjh85j37hj5242"),
            ("dupl@ica.te", "saqewry89r"),
        ] {
            let response = post("register", json!({"email": email, "password": password})).await;
            assert_eq!(response.status_code, StatusCode::CONFLICT);
        }

        Ok(())
    })
    .await
}
