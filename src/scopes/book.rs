use crate::{database::Database, models::book::Book};
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;

pub fn book_scope() -> Scope {
    web::scope("/book")
        .route("/create", web::post().to(create_book))
        .route("/get-all", web::get().to(get_books))
        .route("/get/{id}", web::get().to(get_book_by_id))
        .route("/filter-by", web::post().to(filter_by_param))
    // .route("/{id}", web::put().to(update_book))
    // .route("/{id}", web::delete().to(delete_book))
}

async fn create_book(db: web::Data<Database>, book: web::Json<Book>) -> impl Responder {
    match Book::create(&db, book.into_inner()).await {
        Ok(_) => HttpResponse::Created().json("Book created"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Hiba történt: {}", e)),
    }
}

async fn get_books(db: web::Data<Database>) -> impl Responder {
    let books = Book::get_all(&db).await.unwrap();
    HttpResponse::Ok().json(books)
}

async fn get_book_by_id(db: web::Data<Database>, book_id: web::Path<i32>) -> impl Responder {
    match Book::get_by_id(&db, book_id.into_inner()).await {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(e) => HttpResponse::InternalServerError().json(format!("Hiba történt: {}", e)),
    }
}

#[derive(Deserialize)]
struct FilterInfoJson {
    content: String,
}

async fn filter_by_param(
    db: web::Data<Database>,
    data: web::Json<FilterInfoJson>,
) -> impl Responder {
    match Book::filter_by(&db, &data.content).await {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => HttpResponse::InternalServerError().json(format!("Hiba történt: {}", e)),
    }
}
