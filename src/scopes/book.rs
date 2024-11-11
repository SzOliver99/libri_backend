use crate::{database::Database, models::book::Book};
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;
use std::env;

pub fn book_scope() -> Scope {
    web::scope("/books")
        .route("/", web::post().to(create_book))
        .route("/", web::get().to(get_books))
        .route("/{id}", web::get().to(get_book_by_id))
        .route("/filter-by", web::post().to(filter_by_param))
    // .route("/{id}", web::put().to(update_book))
    // .route("/{id}", web::delete().to(delete_book))
}

async fn create_book(book: web::Json<Book>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Book::create(&mut db, book.into_inner()).await {
        Ok(_) => HttpResponse::Created().json("Book created"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error creating book: {:?}", e)),
    }
}

async fn get_books() -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let books = Book::get_all(&mut db).await.unwrap();
    HttpResponse::Ok().json(books)
}

async fn get_book_by_id(book_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Book::get_by_id(&mut db, book_id.into_inner()).await {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error fetching book: {:?}", e)),
    }
}

#[derive(Deserialize)]
struct FilterInfoJson {
    content: String,
}

async fn filter_by_param(data: web::Json<FilterInfoJson>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Book::filter_by(&mut db, &data.content).await {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error fetching book: {:?}", e)),
    }
}