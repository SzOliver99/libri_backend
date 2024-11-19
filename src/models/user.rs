use super::cart::Cart;
use super::cart::CartBook;
use crate::database::Database;
use crate::scopes::user::ChangeUsernameJson;
use crate::scopes::user::{ChangeBillingInformationJson, ChangeEmailJson, ChangePersonalInformationJson};
use crate::utils::credentials_hashing;
use crate::utils::email;

use serde::Serialize;
use sqlx::prelude::FromRow;
use std::error::Error;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum UserGroup {
    User,
    Admin,
    None,
}

// this is what bothers me
impl From<String> for UserGroup {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Admin" => UserGroup::Admin,
            "User" => UserGroup::User,
            _ => UserGroup::User,
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub group: UserGroup,
}

#[derive(Debug, Serialize, FromRow)]
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
    pub async fn new(db: &mut Database, user: User) -> Result<Self, Box<dyn Error>> {
        // Check if any required fields are null or empty
        if user.username.is_none()
            || user.password.is_none()
            || user.email.is_none()
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
        let hashed_password = credentials_hashing::hash_password(&user.password.unwrap());
        let result = sqlx::query!(
            r#"INSERT INTO users(username, email, password) VALUES(?, ?, ?)"#,
            user.username,
            user.email,
            hashed_password
        )
        .execute(&db.pool)
        .await?;

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
                match credentials_hashing::verify_password(&user.password.unwrap(),
                hashed_user.password.as_ref().unwrap()) {
                    true => Ok(Self {
                    id: hashed_user.id,
                    email: hashed_user.email,
                    username: hashed_user.username,
                    password: Some(hashed_user.password.unwrap()),
                    group: hashed_user.group,
                    }),
                    false => Err("Password not valid".into())
                }
            }
            None => Err("User not found".into()),
        }
    }

    pub async fn get_cart(db: &mut Database, user_id: i32) -> Result<Cart, Box<dyn Error>> {
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE userId = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;

        match cart {
            Some(cart) => {
                let books = sqlx::query_as!(
                    CartBook,
                    r#"
                    SELECT book.id, book.title, book.author, book.price, book.isbn, cart_items.quantity
                    FROM books book
                    JOIN cart_items ON book.id = cart_items.bookId
                    JOIN user_cart ON cart_items.cartId = user_cart.id
                    WHERE user_cart.userId = ?
                    "#,
                    user_id
                )
                .fetch_all(&db.pool)
                .await?;

                Ok(Cart {
                    id: Some(cart.id),
                    user_id: cart.userId,
                    books,
                })
            }
            None => {
                // Create a new cart if user doesn't have one
                Cart::create(db, user_id).await?;

                // Get the newly created cart
                Box::pin(Self::get_cart(db, user_id)).await
            }
        }
    }

    pub async fn forgot_password(
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

    pub async fn reset_password(
        db: &mut Database,
        token: String,
        new_password: String,
    ) -> Result<(), Box<dyn Error>> {
        // Verify the reset token
        let user_token = sqlx::query!(
            "SELECT * FROM reset_tokens WHERE token = ? AND tokenExpires > DATE_SUB(NOW(), INTERVAL 1 HOUR)",
            token
        )
        .fetch_optional(&db.pool)
        .await?;

        match user_token {
            Some(user) => {
                // Update the user's password
                let hashed_password = credentials_hashing::hash_password(&new_password);
                let _ = sqlx::query!(
                    "UPDATE users SET password = ? WHERE id = ?",
                    hashed_password,
                    &user.userId
                )
                .execute(&db.pool)
                .await;

                let _ = sqlx::query!("DELETE FROM reset_tokens WHERE userId = ?", user.userId)
                    .execute(&db.pool)
                    .await;

                Ok(())
            }
            None => Err("Invalid or expired reset token".into()),
        }
    }

    pub async fn change_password(
        db: &mut Database,
        user_id: i32,
        old_password: String,
        new_password: String,
    ) -> Result<(), Box<dyn Error>> {
        // Fetch the user to verify the old password
        let user = sqlx::query!("SELECT * FROM users WHERE id = ?", user_id)
            .fetch_optional(&db.pool)
            .await?;

        match user {
            Some(user) => {
                // Verify the old password
                if !credentials_hashing::verify_password(&old_password, &user.password) {
                    return Err("Old password is incorrect".into());
                }

                // Update the user's password
                let hashed_password = credentials_hashing::hash_password(&new_password);
                let _ = sqlx::query!(
                    "UPDATE users SET password = ? WHERE id = ?",
                    hashed_password,
                    user_id
                )
                .execute(&db.pool)
                .await;

                Ok(())
            }
            None => Err("User not found".into()),
        }
    }

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
    
    pub(crate) async fn change_email(db: &mut Database, id: i32, data: ChangeEmailJson) -> Result<String, Box<dyn Error>> {
        let is_exists = sqlx::query!(
            "SELECT * FROM users WHERE email = ?",
            data.new_email,
        )
        .fetch_optional(&db.pool)
        .await?;

        if is_exists.is_some() {
            return Err("Email already exists".into());
        }
        
        let user_data = sqlx::query_as!(
            Self,
            r#"SELECT * FROM users WHERE id = ?"#,
            id
        )
        .fetch_optional(&db.pool)
        .await?;

        match user_data {
            Some(hashed_user) => {
                match credentials_hashing::verify_password(&data.password,
                hashed_user.password.as_ref().unwrap()) {
                    true => {
                        let _ = sqlx::query!(
                            r#"
                            UPDATE users
                            SET email = ?
                            WHERE id = ?
                            "#,
                            data.new_email,
                            id
                        ).execute(&db.pool).await?;
                        Ok("Email successfully changed!".into())
                    },
                    false => Err("Password not valid".into())
                }
            }
            None => Err("User not found".into()),
        }
    }

    pub(crate) async fn change_username(db: &mut Database, id: i32, data: ChangeUsernameJson) -> Result<String, Box<dyn Error>> {
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
            r#"
            UPDATE users
            SET username = ?
            WHERE id = ?
            "#,
            data.new_username,
            id
        ).execute(&db.pool).await?;
        Ok("Username successfully changed!".into())
    }
    
    pub(crate) async fn is_admin(db: &mut Database, id: i32) -> Result<bool, Box<dyn Error>> {
        let user_role = sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE id = ?"#,
            id
        ).fetch_one(&db.pool).await?;
        let user_group = user_role.group;
        Ok(user_group == UserGroup::Admin)
    }
}
