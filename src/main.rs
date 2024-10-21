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
    Server::run("8080").await?;
    Ok(())
}
