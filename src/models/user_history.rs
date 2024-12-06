use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::database::Database;

use super::cart::{Cart, CartBook};

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
    // books: Option<Vec<CartBook>>,
    price: i32,
    purchase_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionBooks {
    id: u64,
    transaction_history_id: i32,
    book_id: i32,
    quantity: i32,
}

impl TransactionHistory {
    pub async fn create(
        db: &mut Database,
        user_id: i32,
        status: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let books_to_buy = Cart::get_cart(db, user_id).await?.books;

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

        Ok(Self {
            id: transaction.last_insert_id(),
            user_id,
            status: TransactionHistoryStatus::from(status),
            price,
            purchase_date,
        })
    }

    // pub async fn get_all(
    //     db: &mut Database,
    //     user_id: i32,
    // ) -> Result<Vec<TransactionHistory>, Box<dyn Error>> {
    //     let transaction_histories = sqlx::query_as!(
    //         TransactionHistory,
    //         r#"SELECT id, user_id, status, price, purchase_date FROM transaction_history WHERE user_id = ? ORDER BY purchase_date DESC"#,
    //         user_id
    //     )
    //     .fetch_all(&db.pool)
    //     .await?;

    //     // Initialize books to an empty vector for each transaction history
    //     let transaction_histories_with_books: Vec<TransactionHistory> = transaction_histories
    //         .into_iter()
    //         .map(|mut transaction_history| {
    //             transaction_history.books = Some(vec![]); // Set books to an empty vector
    //             transaction_history
    //         })
    //         .collect();

    //     Ok(transaction_histories_with_books)
    // }
}
