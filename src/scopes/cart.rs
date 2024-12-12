use crate::{
    database::Database,
    extractors::authentication_token::AuthenticationToken,
    models::{cart::Cart, user_history::TransactionHistory},
};
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;
use std::env;

pub fn cart_scope() -> Scope {
    web::scope("/cart")
        .route("/{user_id}", web::delete().to(delete_user_cart))
        .route("/book/", web::put().to(increment_book_quantity))
        .route("/book/", web::delete().to(decrease_book_quantity))
        .route("/purchase", web::post().to(buy_user_cart))
}

#[derive(Deserialize)]
struct BookCartRequest {
    book_id: i32,
}

async fn delete_user_cart(auth_token: AuthenticationToken) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::delete_cart(&mut db, auth_token.id as i32).await {
        Ok(_) => HttpResponse::Ok().json("Cart deleted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error deleting cart: {:?}", e)),
    }
}

async fn increment_book_quantity(
    auth_token: AuthenticationToken,
    data: web::Json<BookCartRequest>,
) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::increment_book_quantity(&mut db, auth_token.id as i32, data.book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book added to cart"),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Error adding book to cart: {:?}", e))
        }
    }
}

async fn decrease_book_quantity(
    auth_token: AuthenticationToken,
    data: web::Json<BookCartRequest>,
) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::decrease_book_quantity(&mut db, auth_token.id as i32, data.book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book deleted from cart"),
        Err(e) => HttpResponse::InternalServerError()
            .json(format!("Error deleting book from cart: {:?}", e)),
    }
}

async fn buy_user_cart() -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let transaction = TransactionHistory::create(&mut db, 1, "InProgress")
        .await
        .unwrap();
    HttpResponse::Ok().json(format!("{transaction:?}"))
}
