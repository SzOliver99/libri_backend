use crate::database::Database;

// use crate::services::{auth, book, cart};
use crate::scopes::{self, user};

use actix_web::web;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

pub struct Server;

impl Server {
    pub async fn run(port: &str) -> std::io::Result<()> {
        // Initialize logger if -log flag is passed
        if std::env::args().any(|arg| arg == "-log") {
            env_logger::init_from_env(Env::default().default_filter_or("info"));
        }

        // let redis_url = dotenv::var("REDIS_URL").expect("REDIS_URL must be set");
        let secret_key = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");

        // Create the database
        Database::new(dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .unwrap();

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                // .app_data(web::Data::new(db.clone())) // TODO: Add database directly to the server
                .app_data(web::Data::<String>::new(secret_key.clone()))
                .service(scopes::user::user_scope())
                .service(scopes::book::book_scope())
                .service(scopes::cart::cart_scope())
            // .service(auth::signup)
            // .service(auth::signin)
            // .service(auth::forgot_password)
            // .service(auth::reset_password)
            // .service(book::get_book_by_id)
            // .service(book::create_book)
            // .service(book::buy_book)
            // .service(book::get_user_books)
            // .service(cart::create_cart)
            // .service(cart::get_cart)
            // .service(cart::add_book_to_cart)
            // .service(cart::delete_book_from_cart)
            // .service(cart::delete_cart)
        })
        .bind(format!("127.0.0.1:{port}"))?
        .run()
        .await
    }
}
