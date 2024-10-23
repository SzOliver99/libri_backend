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

#[derive(Debug, Serialize)]
pub struct CartBook {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub price: i32,
    pub description: String,
    pub published_date: String,
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

    pub async fn remove_cart(db: &mut Database, user_id: i32) -> Result<(), Box<dyn Error>> {
        sqlx::query!(r#"DELETE FROM user_cart WHERE userId = ?"#, user_id)
            .execute(&db.pool)
            .await?;
        Ok(())
    }

    pub(crate) async fn add_book_to_cart(
        db: &mut Database,
        user_id: i32,
        book_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        // Check if user has a cart
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE userId = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;
        if cart.is_none() {
            // If user doesn't have a cart, create one
            Self::create(db, user_id).await?;
        }

        // Check if book exists
        let book = sqlx::query!(r#"SELECT * FROM books WHERE id = ?"#, book_id)
            .fetch_optional(&db.pool)
            .await?;

        if book.is_none() {
            return Err("Book doesn't exist".into());
        }

        // Check if the book is already in the cart
        let cart_id = cart.unwrap().id;
        let existing_item = sqlx::query!(
            r#"SELECT * FROM cart_items WHERE cartId = ? AND bookId = ?"#,
            cart_id,
            book_id
        )
        .fetch_optional(&db.pool)
        .await?;

        if let Some(item) = existing_item {
            // If the book is already in the cart, increase the quantity
            sqlx::query!(
                r#"UPDATE cart_items SET quantity = quantity + 1 WHERE id = ?"#,
                item.id
            )
            .execute(&db.pool)
            .await?;
        } else {
            // If the book is not in the cart, insert a new item
            sqlx::query!(
                r#"INSERT INTO cart_items (cartId, bookId, quantity) VALUES (?, ?, 1)"#,
                cart_id,
                book_id
            )
            .execute(&db.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn remove_book_from_cart(
        db: &mut Database,
        user_id: i32,
        book_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE userId = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;

        if let Some(cart) = cart {
            // Get the current quantity of the book in the cart
            let cart_item = sqlx::query!(
                r#"SELECT quantity FROM cart_items WHERE cartId = ? AND bookId = ?"#,
                cart.id,
                book_id
            )
            .fetch_optional(&db.pool)
            .await?;

            if let Some(item) = cart_item {
                if item.quantity > 1 {
                    // Decrease the quantity by 1
                    sqlx::query!(
                        r#"UPDATE cart_items SET quantity = quantity - 1 WHERE cartId = ? AND bookId = ?"#,
                        cart.id,
                        book_id
                    )
                    .execute(&db.pool)
                    .await?;
                } else {
                    // Remove the item if quantity is 1
                    sqlx::query!(
                        r#"DELETE FROM cart_items WHERE cartId = ? AND bookId = ?"#,
                        cart.id,
                        book_id
                    )
                    .execute(&db.pool)
                    .await?;
                }
                Ok(())
            } else {
                Err("Book not found in the cart".into())
            }
        } else {
            Err("User doesn't have a cart".into())
        }
    }

    #[allow(dead_code)]
    pub fn get_total_price(&self) -> i32 {
        self.books.iter().map(|book| book.price).sum()
    }
}
