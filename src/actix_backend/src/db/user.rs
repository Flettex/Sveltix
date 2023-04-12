use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::PgPool;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub allow_login: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
}

pub async fn create_user(
    username: String,
    password: String,
    is_staff: bool,
    is_superuser: bool,
    pool: &PgPool,
) -> User {
    // Superusers are also staff
    let _is_staff = if is_superuser { true } else { is_staff };

    sqlx::query_as!(
        User,
        r#"
            INSERT INTO users ( username, password, is_staff, is_superuser )
            VALUES ($1, $2, $3, $4)
            RETURNING *;
        "#,
        username,
        password,
        _is_staff,
        is_superuser
    )
    .fetch_one(pool)
    .await
    .expect("Failed to create user")
}

pub fn create_password(password: String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(err) => Err(err),
    }
}
