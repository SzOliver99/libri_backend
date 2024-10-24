use crate::database::Database;

use crate::scopes;

use actix_cors::Cors;
use actix_web::{http, web};
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use std::env;

pub struct Server;

impl Server {
    #[actix_web::main]
    pub async fn run(port: u16) -> std::io::Result<()> {
        // Initialize logger if -log flag is passed
        if env::args().any(|arg| arg == "-log") {
            env_logger::init_from_env(Env::default().default_filter_or("info"));
        }

        // let redis_url = dotenv::var("REDIS_URL").expect("REDIS_URL must be set");
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Create the database
        let _db = Database::new(&database_url).await.unwrap();

        HttpServer::new(move || {
            let cors = Cors::default()
                .allowed_origin("https://libri-project.vercel.app")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                    http::header::CONTENT_TYPE,
                ])
                .max_age(3600);

            App::new()
                .wrap(cors)
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
