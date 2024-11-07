use super::cart::Cart;
use super::cart::CartBook;
use crate::database::Database;
use crate::utils::email;
use crate::utils::password;

use serde::Serialize;
use sqlx::prelude::FromRow;
use std::error::Error;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum UserGroup {
    User,
    Admin,
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
#[sqlx(rename_all = "camelCase")]
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
            r#"INSERT INTO users(username, email, password) VALUES(?, ?, ?)"#,
            user.username,
            user.email,
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
            group: UserGroup::User,
        })
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
                if password::verify_password(
                    &user.password.unwrap(),
                    hashed_user.password.as_ref().unwrap(),
                ) {
                    Ok(Self {
                        id: hashed_user.id,
                        username: hashed_user.username,
                        email: hashed_user.email,
                        password: hashed_user.password,
                        group: hashed_user.group,
                    })
                } else {
                    Err("Invalid password".into())
                }
            }
            None => Err("User not found".into()),
        }
    }

    pub async fn get_books(
        db: &mut Database,
        user_id: i32,
    ) -> Result<Vec<UserBooks>, Box<dyn Error>> {
        let user_books = sqlx::query_as!(
            UserBooks,
            r#"SELECT id, userId as user_id, bookId as book_id, status FROM user_books WHERE userId = ?"#,
            user_id
        )
        .fetch_all(&db.pool)
        .await?;

        Ok(user_books)
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

    // pub(crate) async fn forgot_password(
    //     db: &mut Database,
    //     user: User,
    // ) -> Result<(), Box<dyn Error>> {
    //     let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE email = ?", user.email)
    //         .fetch_optional(&db.pool)
    //         .await?;

    //     match user {
    //         Some(user) => {
    //             // Generate a password reset token (you'll need to implement this)
    //             let reset_token = email::generate_reset_token().await;

    //             // Store the reset token in the database (you'll need to implement this)
    //             let _res = email::store_reset_token(db, user.id.unwrap(), &reset_token).await;

    //             // Send the password reset email
    //             email::send_password_reset_email(&user.email.unwrap(), &reset_token).await?;

    //             Ok(())
    //         }
    //         None => Err("User not found".into()),
    //     }
    // }

    // pub(crate) async fn reset_password(
    //     db: &mut Database,
    //     token: String,
    //     new_password: String,
    // ) -> Result<(), Box<dyn Error>> {
    //     // Verify the reset token
    //     let user = sqlx::query!(
    //         "SELECT * FROM reset_tokens WHERE token = ? AND tokenExpires > DATE_SUB(NOW(), INTERVAL 1 HOUR)",
    //         token
    //     )
    //     .fetch_optional(&db.pool)
    //     .await?;

    //     match user {
    //         Some(user) => {
    //             // Update the user's password
    //             let hashed_password = password::hash_password(&new_password);
    //             let _ = sqlx::query!(
    //                 "UPDATE users SET password = ? WHERE id = ?",
    //                 hashed_password,
    //                 &user.userId
    //             )
    //             .execute(&db.pool)
    //             .await;

    //             let _ = sqlx::query!("DELETE FROM reset_tokens WHERE userId = ?", user.userId)
    //                 .execute(&db.pool)
    //                 .await;

    //             Ok(())
    //         }
    //         None => Err("Invalid or expired reset token".into()),
    //     }
    // }
}
