use crate::database::Database;

use std::error::Error;

pub async fn is_exists(db: &mut Database, table: &str, column: &str, value: &str) -> Result<bool, Box<dyn Error>> {
    let result = sqlx::query(&format!("SELECT * FROM {} WHERE {} = ?", table, column))
        .bind(value)
        .fetch_optional(&db.pool)
        .await?;

    Ok(result.is_some())
}

pub async fn is_user_exists(db: &mut Database, user_id: i32) -> Result<bool, Box<dyn Error>> {
    match is_exists(db, "users", "id", user_id.to_string().as_str()).await {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        Err(e) => Err(e),
    }
}
