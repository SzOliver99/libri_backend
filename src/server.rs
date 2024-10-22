use crate::database::Database;

use crate::scopes;

use actix_web::web;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

pub struct Server;

impl Server {
    #[actix_web::main]
    pub async fn run(port: u16) -> std::io::Result<()> {
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
                .app_data(web::Data::<String>::new(secret_key.clone()))
                .service(scopes::user::user_scope())
                .service(scopes::book::book_scope())
                .service(scopes::cart::cart_scope())
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
    }
}
