use crate::database::Database;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub price: i32,
    pub description: String,
    pub image_src: Option<String>,
    pub published_date: String,
    pub isbn: String,
}

impl Book {
    pub async fn create(db: &Database, book: Book) -> Result<(), Box<dyn Error>> {
        // Check if any required fields are null or empty
        if book.title.is_empty()
            || book.author.is_empty()
            || book.description.is_empty()
            || book.image_src.is_none()
            || book.published_date.is_empty()
            || book.isbn.is_empty()
        {
            return Err(
                "Minden mező (title, author, price, description, imageSrc, publishedDate, isbn) kitöltése kötelező"
                    .into(),
            );
        }

        // Check if the book title or isbn already exists
        let existing_book = sqlx::query!(
            r#"SELECT * FROM books WHERE title = ? or isbn = ?"#,
            book.title,
            book.isbn
        )
        .fetch_optional(&db.pool)
        .await?;

        if existing_book.is_some() {
            return Err("A könyv már létezik".into());
        }

        sqlx::query!(
            r#"INSERT INTO books(title, author, price, description, image_src, published_date, isbn) VALUES(?, ?, ?, ?, ?, ?, ?)"#,
            book.title,
            book.author,
            book.price,
            book.description,
            book.image_src.clone().unwrap_or("".to_string()),
            book.published_date,
            book.isbn
        )
        .execute(&db.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all(db: &Database) -> Result<Vec<Book>, Box<dyn Error>> {
        let books = sqlx::query_as!(Book, r#"SELECT * FROM books"#)
            .fetch_all(&db.pool)
            .await?;

        Ok(books)
    }

    pub async fn get_by_id(db: &Database, book_id: i32) -> Result<Book, Box<dyn Error>> {
        let book = sqlx::query_as!(Book, r#"SELECT * FROM books WHERE id = ?"#, book_id)
            .fetch_one(&db.pool)
            .await?;

        Ok(book)
    }

    pub async fn filter_by(db: &Database, query: &str) -> Result<Vec<Book>, Box<dyn Error>> {
        let query_with_whitespace = format!("%{}%", query);
        let query_without_whitespace = format!("%{}%", remove_whitespace(query));
        let books = sqlx::query_as!(
            Book,
            r#"
            SELECT *
            FROM books
            WHERE title LIKE ? OR author LIKE ? OR title LIKE ? OR author LIKE ?
            "#,
            query_with_whitespace,
            query_with_whitespace,
            query_without_whitespace,
            query_without_whitespace
        )
        .fetch_all(&db.pool)
        .await?;

        Ok(books)
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
