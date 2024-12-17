use std::error::Error;

use serde::{Deserialize, Serialize};

use super::cart::Cart;
use crate::database::Database;

#[derive(Debug, Serialize, Deserialize)]
enum TransactionHistoryStatus {
    InProgress,
    Shipping,
    Delivered,
}

impl From<&str> for TransactionHistoryStatus {
    fn from(s: &str) -> Self {
        match s {
            "Delivered" => TransactionHistoryStatus::Delivered,
            "Shipping" => TransactionHistoryStatus::Shipping,
            "InProgress" => TransactionHistoryStatus::InProgress,
            _ => TransactionHistoryStatus::InProgress,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionHistory {
    id: u64,
    user_id: i32,
    status: TransactionHistoryStatus,
    books: Vec<TransactionBooks>,
    price: i32,
    purchase_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionBooks {
    id: u64,
    title: String,
    author: String,
    price: i32,
    description: String,
    image_src: Option<String>,
    published_date: String,
    isbn: String,
    quantity: i32,
}

impl TransactionHistory {
    pub async fn create(
        db: &mut Database,
        user_id: i32,
        status: &str,
    ) -> Result<Self, Box<dyn Error>> {
        // check if books in cart
        let books_to_buy = Cart::get_cart(db, user_id).await?.books;
        println!("{:?}", books_to_buy);
        if books_to_buy.is_empty() {
            return Err("User has no product in cart".into());
        }

        let purchase_date = chrono::Local::now().date_naive();

        let mut price = 0;
        for book in books_to_buy.iter() {
            price += book.price
        }

        let transaction = sqlx::query!(
            r#"INSERT INTO transaction_history(user_id, status, price, purchase_date) VALUES(?, ?, ?, ?)"#,
            user_id,
            status,
            price,
            purchase_date
        )
        .execute(&db.pool)
        .await?;

        for book in books_to_buy.iter() {
            sqlx::query!(r#"INSERT INTO transaction_books(transaction_history_id, book_id, quantity) VALUES(?, ?, ?)"#,
                transaction.last_insert_id(),
                book.id,
                book.quantity
            )
            .execute(&db.pool)
            .await?;
        }

        sqlx::query!(r#"DELETE FROM user_cart WHERE user_id = ?"#, user_id)
            .execute(&db.pool)
            .await?;

        Ok(Self {
            id: transaction.last_insert_id(),
            user_id,
            status: TransactionHistoryStatus::from(status),
            books: vec![],
            price,
            purchase_date,
        })
    }

    pub async fn get_all(
        db: &Database,
        user_id: i32,
    ) -> Result<Vec<TransactionHistory>, Box<dyn Error>> {
        let transaction_histories = sqlx::query!(
            r#"SELECT id, user_id, status, price, purchase_date FROM transaction_history WHERE user_id = ?"#,
            user_id
        )
        .fetch_all(&db.pool)
        .await?;

        let mut result = Vec::new();

        for th in transaction_histories {
            let transaction_histories_books = sqlx::query!(
                r#"
                SELECT DISTINCT b.id, b.title, b.author, b.price, b.description, b.published_date, b.isbn, tb.quantity
                FROM books b
                JOIN transaction_books tb ON tb.book_id = b.id
                WHERE tb.transaction_history_id = ?
                "#,
                th.id
            )
            .fetch_all(&db.pool)
            .await?;

            let transaction_books: Vec<TransactionBooks> = transaction_histories_books
                .into_iter()
                .map(|row| TransactionBooks {
                    id: row.id as u64,
                    title: row.title,
                    author: row.author,
                    price: row.price,
                    description: row.description,
                    image_src: None,
                    published_date: row.published_date,
                    isbn: row.isbn,
                    quantity: row.quantity,
                })
                .collect();

            result.push(TransactionHistory {
                id: th.id as u64,
                user_id: th.user_id,
                status: TransactionHistoryStatus::from(th.status.as_str()),
                books: transaction_books,
                price: th.price,
                purchase_date: th.purchase_date.into(),
            });
        }

        Ok(result)
    }
}
