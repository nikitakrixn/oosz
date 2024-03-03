use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: String,
}

impl User {

    /// Adds a user to the database
    ///
    /// ## Arguments
    /// *
    pub async fn create(username: String, password: String, email: String) -> Result<Self, sqlx::Error> {
        let id = Uuid::new_v4();
        // Логика создания пользователя в базе данных
        // ...
    }

    pub fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).unwrap()
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password).unwrap()
    }
}