use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct SimpleMemoryDatabase {
    users: Arc<Mutex<Vec<User>>>,
}

impl SimpleMemoryDatabase {
    pub fn new() -> Self {
        SimpleMemoryDatabase {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn try_add_user(&self, user: User) -> bool {
        let mut users = self.users.lock().unwrap();

        if users.iter().any(|u| u.email == user.email) {
            // email conflict; cannot add user
            return false;
        }

        users.push(user);
        true
    }

    pub fn validate_user(&self, user: &User) -> bool {
        let users = self.users.lock().unwrap();
        users.iter().any(|u| u.email == user.email && u.password == user.password)
    }
}
