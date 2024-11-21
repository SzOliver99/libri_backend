extern crate redis;
use std::error::Error;
use redis::Commands;

pub struct Redis;

impl Redis {
    pub fn set_token_to_user(con: &mut redis::Connection, user_id: u32, token: &str) -> redis::RedisResult<()> {
        con.set(format!("user:{user_id}"), token)?;
        con.expire(format!("user:{user_id}"), 10)?;

        Ok(())
    }

    fn get_user_token(con: &mut redis::Connection, user_id: u32) -> redis::RedisResult<String> {
        let asd = con.exists::<_, bool>(format!("user:{user_id}"));
        if *asd.as_ref().unwrap() {
            let user_token = con.get(format!("user:{user_id}"))?;
            return Ok(user_token);
        }
        Ok("User token not exists".into())
    }
}