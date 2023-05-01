use axum::async_trait;
use scylla::{prepared_statement::PreparedStatement, Session, SessionBuilder};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use tracing::debug;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[async_trait]
pub trait Database: Clone + Sync + Send {
    async fn new() -> Result<Self, Box<dyn Error>>;
    async fn try_add_user(&self, user: User) -> bool;
    async fn validate_user(&self, user: &User) -> bool;
}

#[derive(Clone)]
pub struct ScyllaDbSession {
    session: Arc<Session>,
    add_user_statement: Arc<PreparedStatement>,
    get_password_statement: Arc<PreparedStatement>,
}

#[async_trait]
impl Database for ScyllaDbSession {
    async fn new() -> Result<Self, Box<dyn Error>> {
        debug!("creating ScyllaDB session");

        let session = SessionBuilder::new()
            .known_node("localhost:9042")
            .build()
            .await?;

        debug!("preparing ScyllaDB statements");

        let add_user_statement = Arc::new(
            session
                .prepare("INSERT INTO axum_api.users (email, password) VALUES (?, ?)")
                .await?,
        );
        let get_password_statement = Arc::new(
            session
                .prepare("SELECT password FROM axum_api.users WHERE email = ?")
                .await?,
        );

        Ok(Self {
            session: Arc::new(session),
            add_user_statement,
            get_password_statement,
        })
    }

    async fn try_add_user(&self, user: User) -> bool {
        self.session
            .execute(&self.add_user_statement, (user.email, user.password))
            .await
            .is_ok()
    }

    async fn validate_user(&self, user: &User) -> bool {
        if let Ok(result) = self
            .session
            .execute(&self.get_password_statement, (&user.email,))
            .await
        {
            &user.password
                == result
                    .first_row()
                    .expect("password query returned 0 rows")
                    .columns
                    .first()
                    .expect("password query row 1 contained 0 columns")
                    .as_ref()
                    .expect("password query row 1 column 1 does not contain a value")
                    .as_text()
                    .expect("password query did not return a string")
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

#[async_trait]
impl Database for SimpleMemoryDatabase {
    async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(SimpleMemoryDatabase {
            users: Arc::new(Mutex::new(Vec::new())),
        })
    }

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
