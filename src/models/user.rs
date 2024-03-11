use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub photo: String,
    pub enabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl User {

    /// Adds a user to the database.
    ///
    /// This method creates a new User instance with the provided username and password,
    /// generates a hash of the password, and sets the remaining fields with default values.
    /// The user is then ready to be inserted into the database.
    ///
    /// ## Arguments
    ///
    /// * pool: The database connection pool
    /// * username: A string representing the username of the user
    /// * password: A string representing the password of the user
    ///
    /// ## Returns
    ///
    /// Returns the identifier of the created user as Uuid.
    pub async fn new(pool: &PgPool, username: String, password: String) -> anyhow::Result<Uuid> {
        let result = sqlx::query!(
        "INSERT INTO accounts (account_name, account_password) VALUES ($1, $2) RETURNING account_id",
            username,
            Self::hash_password(&password)
        )
        .fetch_one(pool)
        .await?;

        let account_id = result.account_id;
        Ok(account_id)
    }

    /// Generates a hashed representation of a password
    ///
    /// This function takes a plain-text password as input and generates a hashed
    /// representation of it using the bcrypt algorithm. The generated hash is then
    /// returned as a string.
    ///
    /// ## Arguments
    /// * `password`: A string representing the plain-text password
    ///
    /// ## Returns
    /// A string representing the hashed password
    pub fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).unwrap()
    }

    /// Verifies if a password matches its hashed representation
    ///
    /// This function takes a plain-text password as input and verifies if it matches
    /// the hashed representation stored in the `User` struct. It uses the bcrypt algorithm
    /// to compare the password with its hashed counterpart. The function returns a boolean
    /// indicating whether the password is a match or not.
    ///
    /// ## Arguments
    /// * `password`: A string representing the plain-text password to verify
    ///
    /// ## Returns
    /// A boolean indicating if the password matches its hashed representation
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password).unwrap()
    }
}