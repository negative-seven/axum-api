use axum::async_trait;
use base64::Engine;
use scylla::{prepared_statement::PreparedStatement, Session, SessionBuilder};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use tokio::join;
use tracing::{debug, error};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[async_trait]
pub trait Database: Clone + Sync + Send {
    async fn try_add_user(&self, user: User) -> bool;
    async fn validate_user(&self, user: &User) -> bool;
}

#[derive(Clone)]
pub struct ScyllaDbSession {
    session: Arc<Session>,
    add_user_statement: Arc<PreparedStatement>,
    get_password_statement: Arc<PreparedStatement>,
}

impl ScyllaDbSession {
    // TODO: make these configurable
    const BCRYPT_COST: u32 = 13;
    const BCRYPT_VERSION_PREFIX: bcrypt::Version = bcrypt::Version::TwoA;
    const BCRYPT_BASE64_ENGINE: base64::engine::GeneralPurpose =
        base64::engine::GeneralPurpose::new(
            &base64::alphabet::BCRYPT,
            base64::engine::general_purpose::NO_PAD,
        );
}

impl ScyllaDbSession {
    pub async fn new(hostnames: &[impl AsRef<str>]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        debug!("creating ScyllaDB session");

        let session = SessionBuilder::new().known_nodes(hostnames).build().await?;

        debug!("preparing ScyllaDB statements");

        let (add_user_statement, get_password_statement) = join!(
            session.prepare(
                "INSERT INTO axum_api.users (email, password_hash, password_salt) VALUES (?, ?, ?)",
            ),
            session
                .prepare("SELECT password_hash, password_salt FROM axum_api.users WHERE email = ?"),
        );

        Ok(Self {
            session: Arc::new(session),
            add_user_statement: Arc::new(add_user_statement?),
            get_password_statement: Arc::new(get_password_statement?),
        })
    }
}

#[async_trait]
impl Database for ScyllaDbSession {
    async fn try_add_user(&self, user: User) -> bool {
        let hashed_password = bcrypt::hash_with_result(user.password, Self::BCRYPT_COST)
            .expect("bcrypt hashing failed");

        self.session
            .execute(
                &self.add_user_statement,
                (
                    user.email,
                    hashed_password.format_for_version(Self::BCRYPT_VERSION_PREFIX),
                    hashed_password.get_salt(),
                ),
            )
            .await
            .is_ok()
    }

    async fn validate_user(&self, user: &User) -> bool {
        if let Ok(result) = self
            .session
            .execute(&self.get_password_statement, (&user.email,))
            .await
        {
            let row = if let Ok(r) = result.first_row() {
                r
            } else {
                return false; // user does not exist
            };

            let row_values = row
                .columns
                .iter()
                .filter_map(Option::as_ref)
                .collect::<Vec<_>>();
            if row_values.len() != 2 {
                error!(
                    "expected 2 elements from password query, got {}",
                    row_values.len()
                );
                return false;
            }
            let password_hash = row_values[0].as_text().expect("password_hash is not text");
            let password_salt = Self::BCRYPT_BASE64_ENGINE
                .decode(
                    row_values[1]
                        .as_text()
                        .expect("password_salt is not text")
                        .as_bytes(),
                )
                .expect("password_salt is not base64-encoded or is not using the bcrypt alphabet")
                .try_into()
                .expect("password_salt is not 16 bytes in size");

            &bcrypt::hash_with_salt(&user.password, Self::BCRYPT_COST, password_salt)
                .expect("bcrypt hashing failed")
                .format_for_version(Self::BCRYPT_VERSION_PREFIX)
                == password_hash
        } else {
            false
        }
    }
}

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct SimpleMemoryDatabase {
    users: Arc<Mutex<Vec<User>>>,
}

impl SimpleMemoryDatabase {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(SimpleMemoryDatabase {
            users: Arc::new(Mutex::new(Vec::new())),
        })
    }
}

#[async_trait]
impl Database for SimpleMemoryDatabase {
    async fn try_add_user(&self, user: User) -> bool {
        let mut users = self.users.lock().unwrap();

        if users.iter().any(|u| u.email == user.email) {
            // email conflict; cannot add user
            return false;
        }

        users.push(user);
        true
    }

    async fn validate_user(&self, user: &User) -> bool {
        let users = self.users.lock().unwrap();
        users
            .iter()
            .any(|u| u.email == user.email && u.password == user.password)
    }
}
