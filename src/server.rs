use crate::database::Database;

use crate::services::{auth_service, book_service};

use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

pub struct Server;

impl Server {
    pub async fn run(port: &str) -> std::io::Result<()> {
        // Initialize logger if -log flag is passed
        if std::env::args().any(|arg| arg == "-log") {
            env_logger::init_from_env(Env::default().default_filter_or("info"));
        }

        // Create the database
        Database::new(dotenv::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .service(auth_service::signup)
                .service(auth_service::signin)
                .service(auth_service::forgot_password)
                .service(auth_service::reset_password)
                .service(book_service::get_book_by_id)
                .service(book_service::create_book)
                .service(book_service::buy_book)
                .service(book_service::get_user_books)
        })
        .bind(format!("127.0.0.1:{port}"))?
        .run()
        .await
    }
}
