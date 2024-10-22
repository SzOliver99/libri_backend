mod extractors;
mod models;
mod scopes;
// mod services;

mod database;

mod utils;

mod server;
use server::Server;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    // Run the server
    Server::run(&dotenv::var("PORT").unwrap_or_else(|_| "8080".to_string())).await?;
    Ok(())
}
