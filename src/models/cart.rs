use super::book::Book;
use crate::{database::Database};
use crate::utils::utils;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cart {
    pub id: Option<i32>,
    pub user_id: i32,
    pub books: Vec<Book>,
}

impl Cart {
    pub async fn new(db: &mut Database, user_id: i32) -> Result<Self, Box<dyn Error>> {
        if !utils::is_user_exists(db, user_id).await? {
            return Err("User not found".into());
        }

        // Check if user has a cart
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE user_id = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;

        if cart.is_some() {
            return Err("User already has a cart".into());
        }

        // Create a new cart
        let result = sqlx::query!(
            r#"INSERT INTO user_cart (user_id) VALUES (?)"#,
            user_id
        )
        .execute(&db.pool)
        .await?;

        let id = result.last_insert_id() as i32;
        Ok(Self {
            id: Some(id),
            user_id,
            books: Vec::new(),
        })
    }

    pub async fn remove_cart(db: &mut Database, user_id: i32) -> Result<(), Box<dyn Error>> {
        sqlx::query!(r#"DELETE FROM user_cart WHERE user_id = ?"#, user_id)
            .execute(&db.pool)
            .await?;
        Ok(())
    }

    pub async fn get_user_cart(db: &mut Database, user_id: i32) -> Result<Cart, Box<dyn Error>> {
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE user_id = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;

        match cart {
            Some(cart) => {
                let books = sqlx::query_as!(
                    Book,
                    r#"
                    SELECT book.*
                    FROM books book
                    JOIN cart_items ON book.id = cart_items.book_id
                    JOIN user_cart ON cart_items.cart_id = user_cart.id
                    WHERE user_cart.user_id = ?
                    "#,
                    user_id
                )
                .fetch_all(&db.pool)
                .await?;
                Ok(Cart {
                    id: Some(cart.id),
                    user_id: cart.user_id,
                    books,
                })
            }
            None => Err("User doesn't have a cart".into()),
        }
    }

    pub(crate) async fn add_book_to_cart(
        db: &mut Database,
        user_id: i32,
        book_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE user_id = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;
        if cart.is_none() {
            return Err("User doesn't have a cart".into());
        }
        let cart_id = cart.unwrap().id;

        let book = sqlx::query!(r#"SELECT * FROM books WHERE id = ?"#, book_id)
            .fetch_optional(&db.pool)
            .await?;

        if book.is_none() {
            return Err("Book doesn't exist".into());
        }

        // Check if the book is already in the cart
        let existing_item = sqlx::query!(
            r#"SELECT * FROM cart_items WHERE cart_id = ? AND book_id = ?"#,
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
                r#"INSERT INTO cart_items (cart_id, book_id, quantity) VALUES (?, ?, 1)"#,
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
        let cart = sqlx::query!(r#"SELECT * FROM user_cart WHERE user_id = ?"#, user_id)
            .fetch_optional(&db.pool)
            .await?;

        if let Some(cart) = cart {
            // Remove the book from cart_items
            let result = sqlx::query!(
                r#"DELETE FROM cart_items WHERE cart_id = ? AND book_id = ?"#,
                cart.id,
                book_id
            )
            .execute(&db.pool)
            .await?;

            if result.rows_affected() == 0 {
                return Err("Book not found in the cart".into());
            }

            Ok(())
        } else {
            Err("User doesn't have a cart".into())
        }
    }

    #[allow(dead_code)]
    pub fn get_total_price(&self) -> i32 {
        self.books.iter().map(|book| book.price).sum()
    }
}
