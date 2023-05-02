mod common;

use common::{post, with_server};
use reqwest::StatusCode;
use serde_json::json;
use serial_test::serial;
use std::error::Error;

#[tokio::test]
#[serial]
async fn login_without_account() -> Result<(), Box<dyn Error>> {
    with_server(async {
        let response = post(
            "login",
            json!({"email": "email@addre.ss", "password": "pw"}),
        )
        .await;
        assert_eq!(response.status_code, StatusCode::UNAUTHORIZED);
        assert!(!response.body.contains_key("token"));

        Ok(())
    })
    .await
}

#[tokio::test]
#[serial]
async fn login_with_wrong_password() -> Result<(), Box<dyn Error>> {
    with_server(async {
        post(
            "register",
            json!({"email": "email@addre.ss", "password": "P_ass1Wo$rD"}),
        )
        .await;

        for password_attempt in [
            "Pass1Wo$rD",
            "P_ass1WorD",
            "P_ass1Wo$RD",
            "P_ass1Wo$rD ",
            " P_ass1Wo$rD",
            "P_ass 1Wo$rD",
            "P__ass1Wo$rD",
        ] {
            let response = post(
                "login",
                json!({"email": "email@addre.ss", "password": password_attempt}),
            )
            .await;

            assert_eq!(response.status_code, StatusCode::UNAUTHORIZED);
            assert!(!response.body.contains_key("token"));
        }

        Ok(())
    })
    .await
}

#[tokio::test]
#[serial]
async fn login_with_correct_password() -> Result<(), Box<dyn Error>> {
    with_server(async {
        post(
            "register",
            json!({"email": "email@addre.ss", "password": "P_ass1Wo$rD"}),
        )
        .await;

        let response = post(
            "login",
            json!({"email": "email@addre.ss", "password": "P_ass1Wo$rD"}),
        )
        .await;

        assert_eq!(response.status_code, StatusCode::OK);
        assert!(response.body.contains_key("token"));

        Ok(())
    })
    .await
}
