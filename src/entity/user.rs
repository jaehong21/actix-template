use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct User {
    // #[serde(skip_serializing)]
    pub id: Option<String>,
    pub username: String,
    pub password: String,
    created_at: String,
    is_delete: bool,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            id: None,
            username,
            password: User::hash_password(password),
            created_at: chrono::Utc::now().to_rfc3339().into(),
            is_delete: false,
        }
    }
    fn hash_password(password: String) -> String {
        hash(password, 10).unwrap()
    }
    pub fn verify_password(&self, password: &str) -> bool {
        match verify(password, &self.password) {
            Ok(b) => b,
            Err(_) => false,
        }
    }
}
