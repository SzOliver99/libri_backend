use crate::database::Database;
use crate::models::book::Book;
use crate::models::user::User;

use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/book/{book_id}")]
pub async fn get_book_by_id(book_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Book::find_by_id(&mut db, book_id.into_inner()).await {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error fetching book: {:?}", e)),
    }
}

#[post("/book")]
pub async fn create_book(book: web::Json<Book>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Book::new(&mut db, book.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Book created"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error creating book: {:?}", e)),
    }
}

#[post("/user/{user_id}/buy-book/{book_id}")]
pub async fn buy_book(path: web::Path<(i32, i32)>) -> impl Responder {
    let (user_id, book_id) = path.into_inner();
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Book::buy_book(&mut db, user_id, book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book bought"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error buying book: {:?}", e)),
    }
}

#[get("/user/{user_id}/books")]
pub async fn get_user_books(user_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match User::find_user_books(&mut db, user_id.into_inner()).await {
        Ok(user_books) => HttpResponse::Ok().json(user_books),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Error fetching user books: {:?}", e))
        }
    }
}
