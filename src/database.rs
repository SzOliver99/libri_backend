#![allow(unused, dead_code)]
use sqlx::{ConnectOptions, FromRow, MySql, MySqlConnection, Pool, Row};
use std::error::Error;
use std::time::Duration;

#[derive(FromRow, Debug, Clone)]
pub struct Database {
    pub pool: Pool<MySql>,
    pub redis: redis::Client
    // pub connection: MySqlConnection,
}

impl Database {
    pub async fn new(sql_url: &str, redis_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(sql_url)
            .await?;

        let redis = redis::Client::open(redis_url).unwrap();
        // let mut redis = client.get_connection().unwrap();

        // sqlx::migrate!("./migrations").run(&pool).await?;

        // let connection = sqlx::mysql::MySqlConnectOptions::new()
        //     .username("root")
        //     .password("bookstore123")
        //     .host("bookstore-database.fly.dev")
        //     .port(3306)
        //     .database("bookstore_db")
        //     .connect()
        //     .await?;

        Ok(Database { pool, redis })
    }
}
