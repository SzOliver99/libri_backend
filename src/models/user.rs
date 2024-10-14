use crate::database::Database;
use crate::utils::password;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBooks {
    pub id: Option<i32>,
    pub user_id: i32,
    pub book_id: i32,
    pub status: String,
}

impl User {
    pub async fn new(db: &mut Database, user: User) -> Result<Self, Box<dyn Error>> {
        // Check if any required fields are null or empty
        if user.username.is_empty()
            || user.password.is_empty()
            || user.email.is_none()
            || user.email.as_ref().unwrap().is_empty()
        {
            return Err("All fields (email, username, password) are required".into());
        }

        // Check if user already exists
        let existing_user = sqlx::query!(
            "SELECT * FROM users WHERE email = ? OR username = ?",
            user.email,
            user.username
        )
        .fetch_optional(&db.pool)
        .await?;

        if existing_user.is_some() {
            return Err("User already exists".into());
        }

        // If user doesn't exist, create a new one
        let hashed_password = password::hash_password(&user.password);
        let result = sqlx::query!(
            "INSERT INTO users(email, username, password) VALUES(?, ?, ?)",
            user.email,
            user.username,
            hashed_password
        )
        .execute(&db.pool)
        .await?;

        let id = result.last_insert_id() as i32;
        Ok(Self {
            id: Some(id),
            username: user.username,
            email: user.email,
            password: hashed_password,
        })
    }

    pub async fn login_with_password(
        db: &mut Database,
        user: User,
    ) -> Result<Self, Box<dyn Error>> {
        let user_data = sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE username = ?",
            user.username
        )
        .fetch_optional(&db.pool)
        .await?;

        match user_data {
            Some(hashed_user) => {
                if password::verify_password(&user.password, &hashed_user.password) {
                    Ok(user)
                } else {
                    Err("Invalid password".into())
                }
            }
            None => Err("User not found".into()),
        }
    }

    pub async fn find_user(
        db: &mut Database,
        email: String,
        username: String,
    ) -> Result<Self, Box<dyn Error>> {
        let user = sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE email = ? OR username = ?",
            &email,
            &username
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_books(
        db: &mut Database,
        user_id: i32,
    ) -> Result<Vec<UserBooks>, Box<dyn Error>> {
        let user_books = sqlx::query_as!(
            UserBooks,
            "SELECT * FROM user_books WHERE user_id = ?",
            user_id
        )
        .fetch_all(&db.pool)
        .await?;

        Ok(user_books)
    }
}
