use crate::database::Database;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub price: i32,
    pub description: String,
    pub published_date: String,
    pub isbn: String,
}

impl Book {
    pub async fn new(db: &mut Database, book: Book) -> Result<Book, Box<dyn Error>> {
        // Check if any required fields are null or empty
        if book.title.is_empty()
            || book.author.is_empty()
            || book.description.is_empty()
            || book.published_date.is_empty()
            || book.isbn.is_empty()
        {
            return Err(
                "All fields (title, author, price, description, published_date, isbn) are required"
                    .into(),
            );
        }

        // Check if the book title or isbn already exists
        let existing_book = sqlx::query!(
            "SELECT * FROM books WHERE title = ? or isbn = ?",
            book.title,
            book.isbn
        )
        .fetch_optional(&db.pool)
        .await?;

        if existing_book.is_some() {
            return Err("Book already exists".into());
        }

        let result = sqlx::query!(
            "INSERT INTO books(title, author, price, description, published_date, isbn) VALUES(?, ?, ?, ?, ?, ?)",
            book.title,
            book.author,
            book.price,
            book.description,
            book.published_date,
            book.isbn
        )
        .execute(&db.pool)
        .await?;

        let id = result.last_insert_id() as i32;
        Ok(Book {
            id: Some(id),
            title: book.title,
            author: book.author,
            price: book.price,
            description: book.description,
            published_date: book.published_date,
            isbn: book.isbn,
        })
    }

    pub async fn find_by_id(db: &mut Database, book_id: i32) -> Result<Book, Box<dyn Error>> {
        let book = sqlx::query_as!(Book, "SELECT * FROM books WHERE id = ?", book_id)
            .fetch_one(&db.pool)
            .await?;

        Ok(book)
    }

    pub async fn buy_book(
        db: &mut Database,
        user_id: i32,
        book_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            "INSERT INTO user_books(user_id, book_id, status) VALUES(?, ?, ?)",
            user_id,
            book_id,
            "purchased"
        )
        .execute(&db.pool)
        .await?;

        // sqlx::query!("UPDATE books SET stock = stock - 1 WHERE id = ?", book_id)
        //     .execute(&db.pool)
        //     .await?;

        Ok(())
    }
}
