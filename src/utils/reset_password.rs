use actix_web::web;
use jsonwebtoken::{decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};

use crate::extractors::authentication_token::Claims;

use super::jwt::DecodeBody;

pub(crate) fn generate_reset_password_token(user_id: usize, reset_secret: &str) -> Result<String, JwtError> {
    let claims = Claims {
        id: user_id,
        exp: (chrono::Utc::now() + chrono::Duration::minutes(15)).timestamp() as usize, // Token valid for 15 minutes
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(reset_secret.as_ref()))?;
    Ok(token)
}

pub(crate) fn validate_reset_password_token(body: web::Json<DecodeBody>, reset_secret: web::Data<String>,) -> Result<Claims, JwtError> {
    let token_result: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
        &body.token, 
        &DecodingKey::from_secret(reset_secret.as_str().as_ref()), 
        &Validation::new(Algorithm::HS256));
        
    match token_result {
        Ok(token) => Ok(token.claims.id),
        Err(e) => Err(e),
    }
}