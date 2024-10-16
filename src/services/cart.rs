use crate::database::Database;
use crate::models::cart::Cart;

use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[post("/cart/{user_id}")]
pub async fn create_cart(user_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    println!("Creating cart for user: {}", user_id);
    match Cart::new(&mut db, user_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Cart created"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error creating cart: {:?}", e)),
    }
}

#[get("/cart/{user_id}")]
pub async fn get_cart(user_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::get_user_cart(&mut db, user_id.into_inner()).await {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error getting cart: {:?}", e)),
    }
}

#[post("/cart/{user_id}/book/{book_id}")]
pub async fn add_book_to_cart(path: web::Path<(i32, i32)>) -> impl Responder {
    let (user_id, book_id) = path.into_inner();
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::add_book_to_cart(&mut db, user_id, book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book added to cart"),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Error adding book to cart: {:?}", e))
        }
    }
}

#[delete("/cart/{user_id}")]
pub async fn delete_cart(user_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::remove_cart(&mut db, user_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Cart deleted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error deleting cart: {:?}", e)),
    }
}

#[delete("/cart/{user_id}/book/{book_id}")]
pub async fn delete_book_from_cart(path: web::Path<(i32, i32)>) -> impl Responder {
    let (user_id, book_id) = path.into_inner();
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Cart::remove_book_from_cart(&mut db, user_id, book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book deleted from cart"),
        Err(e) => HttpResponse::InternalServerError()
            .json(format!("Error deleting book from cart: {:?}", e)),
    }
}
