use crate::database::Database;

use std::error::Error;

pub(crate) fn string_to_fixed_array(input: &str) -> Result<[u8; 32], &'static str> {
    let bytes = input.as_bytes();
    
    if bytes.len() > 32 {
        return Err("Input string is too long; must be 32 bytes or less.");
    }
    
    let mut array = [0u8; 32]; // Create a fixed-size array of 32 bytes
    array[..bytes.len()].copy_from_slice(bytes); // Copy the bytes into the array
    Ok(array) // Return the array
}

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
