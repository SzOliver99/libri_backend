use crate::database::Database;
use crate::utils::utils;

use serde::Serialize;
use sqlx::prelude::FromRow;
use std::error::Error;

#[derive(Debug, Serialize, FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct Cart {
    pub id: Option<i32>,
    pub user_id: i32,
    pub books: Vec<CartBook>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CartBook {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub price: i32,
    pub isbn: String,
    pub quantity: i32,
}

impl Cart {
    pub async fn create(db: &mut Database, user_id: i32) -> Result<(), Box<dyn Error>> {
        if !utils::is_user_exists(db, user_id).await? {
            return Err("User not found".into());
        }

        // Check if user has a cart
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE userId = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;

        if cart.is_some() {
            return Err("User already has a cart".into());
        }

        // Create a new cart
        sqlx::query!(r#"INSERT INTO user_cart (userId) VALUES (?)"#, user_id)
            .execute(&db.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_cart(db: &mut Database, user_id: i32) -> Result<(), Box<dyn Error>> {
        sqlx::query!(r#"DELETE FROM user_cart WHERE userId = ?"#, user_id)
            .execute(&db.pool)
            .await?;
        Ok(())
    }

    pub(crate) async fn increment_book_quantity(
        db: &mut Database,
        user_id: i32,
        book_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        // Check if user has a cart
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE userId = ?"#, user_id)
            .fetch_one(&db.pool)
            .await?;

        // Check if book exists
        if sqlx::query!(r#"SELECT * FROM books WHERE id = ?"#, book_id)
            .fetch_optional(&db.pool)
            .await?
            .is_none()
        {
            return Err("Book doesn't exist".into());
        }

        // Upsert the cart item
        sqlx::query!(
            r#"
            INSERT INTO cart_items (cartId, bookId, quantity)
            VALUES (?, ?, 1)
            ON DUPLICATE KEY UPDATE quantity = quantity + 1
            "#,
            cart.id,
            book_id
        )
        .execute(&db.pool)
        .await?;

        Ok(())
    }

    pub async fn decrease_book_quantity(
        db: &mut Database,
        user_id: i32,
        book_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE userId = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;

        // Check if user has a cart
        if let Some(cart) = cart {
            sqlx::query!(
                r#"
                UPDATE cart_items
                SET quantity = CASE
                    WHEN quantity > 1 THEN quantity - 1
                    ELSE quantity
                END
                WHERE cartId = ? AND bookId = ?
                "#,
                cart.id,
                book_id
            )
            .execute(&db.pool)
            .await?;

            // If quantity becomes 0, remove the item
            sqlx::query!(
                r#"
                DELETE FROM cart_items
                WHERE cartId = ? AND bookId = ? AND quantity = 0
                "#,
                cart.id,
                book_id
            )
            .execute(&db.pool)
            .await?;
            Ok(())
        } else {
            Err("User doesn't have a cart".into())
        }
    }
}
