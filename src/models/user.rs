use crate::database::Database;
use crate::utils::email;
use crate::utils::password;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub group: Option<String>,
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
        if user.username.is_none()
            || user.password.is_none()
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
        let hashed_password = password::hash_password(&user.password.unwrap());
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
            email: user.email,
            username: user.username,
            password: Some(hashed_password),
            group: None,
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
                if password::verify_password(
                    &user.password.unwrap(),
                    hashed_user.password.as_ref().unwrap(),
                ) {
                    Ok(hashed_user)
                } else {
                    Err("Invalid password".into())
                }
            }
            None => Err("User not found".into()),
        }
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

    pub(crate) async fn forgot_password(
        db: &mut Database,
        user: User,
    ) -> Result<(), Box<dyn Error>> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE email = ?", user.email)
            .fetch_optional(&db.pool)
            .await?;

        match user {
            Some(user) => {
                // Generate a password reset token (you'll need to implement this)
                let reset_token = email::generate_reset_token().await;

                // Store the reset token in the database (you'll need to implement this)
                let _res = email::store_reset_token(db, user.id.unwrap(), &reset_token).await;

                // Send the password reset email
                email::send_password_reset_email(&user.email.unwrap(), &reset_token).await?;

                Ok(())
            }
            None => Err("User not found".into()),
        }
    }

    pub(crate) async fn reset_password(
        db: &mut Database,
        token: String,
        new_password: String,
    ) -> Result<(), Box<dyn Error>> {
        // Verify the reset token
        let user = sqlx::query!(
            "SELECT * FROM reset_tokens WHERE token = ? AND token_expires > DATE_SUB(NOW(), INTERVAL 1 HOUR)",
            token
        )
        .fetch_optional(&db.pool)
        .await?;

        match user {
            Some(user) => {
                // Update the user's password
                let hashed_password = password::hash_password(&new_password);
                let _ = sqlx::query!(
                    "UPDATE users SET password = ? WHERE id = ?",
                    hashed_password,
                    &user.user_id
                )
                .execute(&db.pool)
                .await;

                let _ = sqlx::query!("DELETE FROM reset_tokens WHERE user_id = ?", user.user_id)
                    .execute(&db.pool)
                    .await;

                Ok(())
            }
            None => Err("Invalid or expired reset token".into()),
        }
    }
}
