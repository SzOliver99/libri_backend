extern crate redis;
use redis::Commands;

pub struct Redis;

impl Redis {
    pub fn set_token_to_user(
        con: &mut redis::Connection,
        user_id: u32,
        token: &str,
    ) -> redis::RedisResult<()> {
        con.set::<_, _, String>(token, format!("user:{user_id}"))?;
        con.expire::<_, ()>(token, 30)?;

        Ok(())
    }

    pub fn get_user_id_by_token(
        con: &mut redis::Connection,
        token: &str,
    ) -> redis::RedisResult<i32> {
        let is_exists = con.exists::<_, bool>(&token)?;
        if is_exists {
            let redis_value = con.get::<_, String>(&token)?;
            let user_id = redis_value[5..].parse::<i32>().unwrap();
            return Ok(user_id);
        }
        Ok(-1) // Not exists
    }
}
