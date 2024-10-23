use crate::{database::Database, models::cart::Cart};
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct BookCartRequest {
    user_id: i32,
    book_id: i32,
}

pub fn cart_scope() -> Scope {
    web::scope("/cart")
        .route("/{user_id}", web::delete().to(delete_user_cart))
        .route("/book/", web::put().to(increment_book_quantity))
        .route("/book/", web::delete().to(decrease_book_quantity))
}

async fn delete_user_cart(user_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::delete_cart(&mut db, user_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Cart deleted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error deleting cart: {:?}", e)),
    }
}

async fn increment_book_quantity(data: web::Json<BookCartRequest>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::increment_book_quantity(&mut db, data.user_id, data.book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book added to cart"),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Error adding book to cart: {:?}", e))
        }
    }
}

async fn decrease_book_quantity(data: web::Json<BookCartRequest>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::decrease_book_quantity(&mut db, data.user_id, data.book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book deleted from cart"),
        Err(e) => HttpResponse::InternalServerError()
            .json(format!("Error deleting book from cart: {:?}", e)),
    }
}
