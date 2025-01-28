use crate::{
    database::Database,
    extractors::authentication_token::AuthenticationToken,
    models::{cart::Cart, user_history::TransactionHistory},
};
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;

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

async fn delete_user_cart(
    db: web::Data<Database>,
    auth_token: AuthenticationToken,
) -> impl Responder {
    match Cart::delete_cart(&db, auth_token.id as i32).await {
        Ok(_) => HttpResponse::Ok().json("Kosár sikeresen törölve."),
        Err(e) => HttpResponse::InternalServerError().json(format!("Hiba történt: {}", e)),
    }
}

async fn increment_book_quantity(
    db: web::Data<Database>,
    auth_token: AuthenticationToken,
    data: web::Json<BookCartRequest>,
) -> impl Responder {
    match Cart::increment_book_quantity(&db, auth_token.id as i32, data.book_id).await {
        Ok(_) => HttpResponse::Ok().json(""),
        Err(e) => HttpResponse::InternalServerError().json(format!("Hiba történt: {}", e)),
    }
}

async fn decrease_book_quantity(
    db: web::Data<Database>,
    auth_token: AuthenticationToken,
    data: web::Json<BookCartRequest>,
) -> impl Responder {
    match Cart::decrease_book_quantity(&db, auth_token.id as i32, data.book_id).await {
        Ok(_) => HttpResponse::Ok().json(""),
        Err(e) => HttpResponse::InternalServerError().json(format!("Hiba történt: {}", e)),
    }
}

async fn buy_user_cart(db: web::Data<Database>, auth_token: AuthenticationToken) -> impl Responder {
    match TransactionHistory::create(&db, auth_token.id as i32, "InProgress").await {
        Ok(_) => HttpResponse::Ok()
            .json("Megkaptuk a rendelését, további információkért e-mailt küldünk Önnek."),
        Err(e) => HttpResponse::InternalServerError().json(format!("Hiba történt: {}", e)),
    }
}
