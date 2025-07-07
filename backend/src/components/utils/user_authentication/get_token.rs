use actix_web::{HttpRequest, Error, error::ErrorUnauthorized};


pub fn get_token(req: &HttpRequest) -> Result<String, Error> {
    if let Some(header_value) = req.headers().get("Authorization") {
        if let Ok(header_str) = header_value.to_str() {
            if let Some(token) = header_str.strip_prefix("Bearer ") {
                return Ok(token.to_string());
            }
        }
    }

    Err(ErrorUnauthorized("Invalid or missing token"))
}