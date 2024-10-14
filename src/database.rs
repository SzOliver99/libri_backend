#![allow(unused, dead_code)]
use sqlx::{ConnectOptions, FromRow, MySql, MySqlConnection, Pool, Row};
use std::error::Error;

#[derive(FromRow, Debug)]
pub struct Database {
    pub pool: Pool<MySql>,
    pub connection: MySqlConnection,
}

impl Database {
    pub async fn new(url: String) -> Result<Self, Box<dyn Error>> {
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(5) // You can adjust the number of connections as needed
            .connect(&url)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        let connection = sqlx::mysql::MySqlConnectOptions::new()
            .username("root")
            .host("localhost")
            .port(3306)
            .database("task_manager")
            .connect()
            .await?;

        Ok(Database { pool, connection })
    }
}
