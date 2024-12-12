// src/models/user.rs

extern crate redis;
use redis::Commands;

use crate::database::Database;
use crate::scopes::user::{
    ChangeBillingInformationJson, ChangeEmailJson, ChangePersonalInformationJson,
    ChangeUsernameJson,
};
use crate::utils::{
    credentials_hashing,
    email::{Email, Token},
    redis::Redis,
};

use serde::{Deserialize, Serialize};
use std::error::Error;

// Enum representing user groups
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserGroup {
    User,
    Admin,
    None,
}

// Implement conversion from String to UserGroup
impl From<String> for UserGroup {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Admin" => UserGroup::Admin,
            "User" => UserGroup::User,
            _ => UserGroup::User,
        }
    }
}

// User struct representing a user in the system
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub group: UserGroup,
}

// UserInfo struct representing additional user information
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub billing_address: String,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub username: String,
    pub email: String,
}

impl User {
    // Create a new user
    pub async fn new(db: &mut Database, user: User) -> Result<Self, Box<dyn Error>> {
        // Check for required fields
        if user.username.is_none() || user.password.is_none() || user.email.is_none() {
            return Err("All fields (email, username, password) are required".into());
        }

        // Check if user already exists
        if Self::is_user_exists(db, user.id.unwrap()).await? {
            return Err("User already exists".into());
        }

        // Hash the password and insert the new user
        let hashed_password = credentials_hashing::hash_password(&user.password.unwrap());
        let result = sqlx::query!(
            r#"INSERT INTO users(username, email, password) VALUES(?, ?, ?)"#,
            user.username,
            user.email,
            hashed_password
        )
        .execute(&db.pool)
        .await?;

        // Insert user info
        let _ = sqlx::query!(
            r#"INSERT INTO user_info(user_id) VALUES(?)"#,
            result.last_insert_id()
        )
        .execute(&db.pool)
        .await?;

        let id = result.last_insert_id() as i32;
        Ok(Self {
            id: Some(id),
            email: user.email,
            username: user.username,
            password: Some(hashed_password),
            group: UserGroup::User,
        })
    }

    // Get user information
    pub async fn get_info(db: &mut Database, user_id: i32) -> Result<UserInfo, Box<dyn Error>> {
        let user_info = sqlx::query_as!(
            UserInfo,
            r#"
            SELECT first_name, last_name, phone_number, billing_address, city, state_province, postal_code, email, username
            FROM user_info
            JOIN users ON user_info.user_id = users.id
            WHERE user_info.user_id = ?
            "#,
            user_id
        )
        .fetch_optional(&db.pool)
        .await?;

        match user_info {
            Some(info) => Ok(info),
            None => Err("User info not found".into()),
        }
    }

    // Login with username and password
    pub async fn login_with_password(
        db: &mut Database,
        user: User,
    ) -> Result<Self, Box<dyn Error>> {
        let user_data = sqlx::query_as!(
            Self,
            r#"SELECT * FROM users WHERE username = ?"#,
            user.username
        )
        .fetch_optional(&db.pool)
        .await?;

        match user_data {
            Some(hashed_user) => {
                if credentials_hashing::verify_password(
                    &user.password.unwrap(),
                    hashed_user.password.as_ref().unwrap(),
                ) {
                    Ok(Self {
                        id: hashed_user.id,
                        email: hashed_user.email,
                        username: hashed_user.username,
                        password: Some(hashed_user.password.unwrap()),
                        group: hashed_user.group,
                    })
                } else {
                    Err("Password not valid".into())
                }
            }
            None => Err("User not found".into()),
        }
    }

    // Login with email and token
    pub(crate) async fn login_with_email(
        db: &mut Database,
        redis_con: &mut redis::Connection,
        code: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let user_id = Redis::get_user_id_by_token(redis_con, code)?;

        let user_data = sqlx::query_as!(Self, r#"SELECT * FROM users WHERE id = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;

        match user_data {
            Some(hashed_user) => {
                redis_con.del::<_, String>(code)?;
                Ok(Self {
                    id: hashed_user.id,
                    email: hashed_user.email,
                    username: hashed_user.username,
                    password: Some(hashed_user.password.unwrap()),
                    group: hashed_user.group,
                })
            }
            None => Err("User not found".into()),
        }
    }

    // Send authentication code for password reset
    pub async fn send_authentication_code(
        db: &mut Database,
        redis_con: &mut redis::Connection,
        user: User,
    ) -> Result<(), Box<dyn Error>> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE email = ?", user.email)
            .fetch_optional(&db.pool)
            .await?;

        match user {
            Some(user) => {
                let reset_token = Token::generate_six_digit_number();
                Redis::set_token_to_user(redis_con, user.id.unwrap() as u32, &reset_token)?;
                Email::send_authentication_code(&user.email.unwrap(), &reset_token).await?;
                Ok(())
            }
            None => Err("Email not found".into()),
        }
    }

    // Handle password reset request
    pub async fn forgot_password(
        db: &mut Database,
        redis_con: &mut redis::Connection,
        user: User,
    ) -> Result<(), Box<dyn Error>> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE email = ?", user.email)
            .fetch_optional(&db.pool)
            .await?;

        match user {
            Some(user) => {
                let reset_token = Token::generate_reset_token();
                Redis::set_token_to_user(redis_con, user.id.unwrap() as u32, &reset_token)?;
                Email::send_password_reset_email(&user.email.unwrap(), &reset_token).await?;
                Ok(())
            }
            None => Err("User not found".into()),
        }
    }

    // Reset user's password
    pub async fn reset_password(
        db: &mut Database,
        redis_con: &mut redis::Connection,
        reset_token: String,
        new_password: String,
    ) -> Result<(), Box<dyn Error>> {
        let user_id = Redis::get_user_id_by_token(redis_con, &reset_token)?;
        if user_id == -1 {
            return Err("Token does not exist".into());
        }

        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE id = ?", user_id)
            .fetch_optional(&db.pool)
            .await?;

        match user {
            Some(user) => {
                let hashed_password = credentials_hashing::hash_password(&new_password);
                let _ = sqlx::query!(
                    "UPDATE users SET password = ? WHERE id = ?",
                    hashed_password,
                    &user.id
                )
                .execute(&db.pool)
                .await?;

                redis_con.del::<_, ()>(&reset_token)?;
                Ok(())
            }
            None => Err("Invalid or expired reset token".into()),
        }
    }

    // Change user's password
    pub async fn change_password(
        db: &mut Database,
        user_id: i32,
        old_password: String,
        new_password: String,
    ) -> Result<(), Box<dyn Error>> {
        let user = sqlx::query!("SELECT * FROM users WHERE id = ?", user_id)
            .fetch_optional(&db.pool)
            .await?;

        match user {
            Some(user) => {
                if !credentials_hashing::verify_password(&old_password, &user.password) {
                    return Err("Old password is incorrect".into());
                }

                let hashed_password = credentials_hashing::hash_password(&new_password);
                let _ = sqlx::query!(
                    "UPDATE users SET password = ? WHERE id = ?",
                    hashed_password,
                    user_id
                )
                .execute(&db.pool)
                .await?;

                Ok(())
            }
            None => Err("User not found".into()),
        }
    }

    // Change user's personal information
    pub async fn change_personal_info(
        db: &mut Database,
        id: i32,
        data: ChangePersonalInformationJson,
    ) -> Result<(), Box<dyn Error>> {
        let _ = sqlx::query!(
            r#"
            UPDATE user_info
            SET first_name = ?, last_name = ?, phone_number = ?
            WHERE user_id = ?
            "#,
            data.first_name,
            data.last_name,
            data.phone_number,
            id,
        )
        .execute(&db.pool)
        .await?;

        Ok(())
    }

    // Change user's billing information
    pub async fn change_billing_info(
        db: &mut Database,
        id: i32,
        data: ChangeBillingInformationJson,
    ) -> Result<(), Box<dyn Error>> {
        let _ = sqlx::query!(
            r#"
            UPDATE user_info
            SET billing_address = ?, city = ?, state_province = ?, postal_code = ?
            WHERE user_id = ?
            "#,
            data.billing_address,
            data.city,
            data.state_province,
            data.postal_code,
            id,
        )
        .execute(&db.pool)
        .await?;

        Ok(())
    }

    // Delete user account
    pub async fn delete_account(db: &mut Database, id: i32) -> Result<(), Box<dyn Error>> {
        let _ = sqlx::query!(
            r#"
                DELETE FROM users
                WHERE id = ?
                "#,
            id,
        )
        .execute(&db.pool)
        .await?;

        Ok(())
    }

    // Change user's email
    pub(crate) async fn change_email(
        db: &mut Database,
        id: i32,
        data: ChangeEmailJson,
    ) -> Result<(), Box<dyn Error>> {
        let is_exists = sqlx::query!("SELECT * FROM users WHERE email = ?", data.new_email)
            .fetch_optional(&db.pool)
            .await?;

        if is_exists.is_some() {
            return Err("Email already exists".into());
        }

        let user_data = sqlx::query_as!(Self, r#"SELECT * FROM users WHERE id = ?"#, id)
            .fetch_optional(&db.pool)
            .await?;

        match user_data {
            Some(hashed_user) => {
                if credentials_hashing::verify_password(
                    &data.password,
                    hashed_user.password.as_ref().unwrap(),
                ) {
                    let _ = sqlx::query!(
                        r#"
                        UPDATE users
                        SET email = ?
                        WHERE id = ?
                        "#,
                        data.new_email,
                        id
                    )
                    .execute(&db.pool)
                    .await?;
                    Ok(())
                } else {
                    Err("Password not valid".into())
                }
            }
            None => Err("User not found".into()),
        }
    }

    // Change user's username
    pub(crate) async fn change_username(
        db: &mut Database,
        id: i32,
        data: ChangeUsernameJson,
    ) -> Result<String, Box<dyn Error>> {
        let is_exists = sqlx::query!(
            r#"SELECT * FROM users WHERE username = ?"#,
            data.new_username,
        )
        .fetch_optional(&db.pool)
        .await?;

        if is_exists.is_some() {
            return Err("Username is taken!".into());
        }

        let _ = sqlx::query!(
            r#"UPDATE users SET username = ? WHERE id = ?"#,
            data.new_username,
            id
        )
        .execute(&db.pool)
        .await?;
        Ok("Username successfully changed!".into())
    }

    // Check if user is an admin
    pub(crate) async fn is_admin(db: &mut Database, id: i32) -> Result<bool, Box<dyn Error>> {
        let user_role = sqlx::query_as!(User, r#"SELECT * FROM users WHERE id = ?"#, id)
            .fetch_one(&db.pool)
            .await?;
        Ok(user_role.group == UserGroup::Admin)
    }

    // Check if a record exists in a specified table
    pub async fn is_exists(
        db: &mut Database,
        table: &str,
        column: &str,
        value: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let result = sqlx::query(&format!("SELECT * FROM {} WHERE {} = ?", table, column))
            .bind(value)
            .fetch_optional(&db.pool)
            .await?;

        Ok(result.is_some())
    }

    // Check if a user exists by user ID
    pub async fn is_user_exists(db: &mut Database, user_id: i32) -> Result<bool, Box<dyn Error>> {
        Self::is_exists(db, "users", "id", user_id.to_string().as_str()).await
    }
}
