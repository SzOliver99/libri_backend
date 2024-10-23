use crate::{database::Database, models::cart::Cart};
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct BookCartRequest {
    user_id: i32,
    book_id: i32,
}

pub fn cart_scope() -> Scope {
    web::scope("/cart")
        .route("/{user_id}", web::post().to(create_user_cart))
        .route("/{user_id}", web::delete().to(delete_user_cart))
        .route("/book", web::post().to(add_book_to_cart))
        .route("/book", web::delete().to(delete_book_from_cart))
    // .route("/{id}", web::put().to(update_book))
    // .route("/{id}", web::delete().to(delete_book))
}

async fn create_user_cart(user_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    println!("Creating cart for user: {}", user_id);
    match Cart::create(&mut db, user_id.into_inner()).await {
        Ok(_) => HttpResponse::Created().json("Cart created"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error creating cart: {:?}", e)),
    }
}

async fn delete_user_cart(user_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::remove_cart(&mut db, user_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Cart deleted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error deleting cart: {:?}", e)),
    }
}

async fn add_book_to_cart(data: web::Json<BookCartRequest>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::add_book_to_cart(&mut db, data.user_id, data.book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book added to cart"),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Error adding book to cart: {:?}", e))
        }
    }
}

async fn delete_book_from_cart(data: web::Json<BookCartRequest>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::remove_book_from_cart(&mut db, data.user_id, data.book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book deleted from cart"),
        Err(e) => HttpResponse::InternalServerError()
            .json(format!("Error deleting book from cart: {:?}", e)),
    }
}
