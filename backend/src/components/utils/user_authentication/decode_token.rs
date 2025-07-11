use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors::Error as JwtError};
use crate::components::authentication::models::Claims;

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false; // ✅ ignore expiration in dev

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}
