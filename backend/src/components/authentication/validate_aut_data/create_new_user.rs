use crate::components::authentication::models::EmailPayload;
use crate::components::db::AsyncConnectionPool;
use crate::components::utils::user_authentication::{
    decode_token::decode_token, get_token::get_token,
};
use actix_web::{Error, HttpRequest, HttpResponse, error::ErrorUnauthorized, web};
use std::sync::Arc;
pub async fn create_new_user(
    req: HttpRequest,
    payload: web::Json<EmailPayload>,
    pool: web::Data<Arc<AsyncConnectionPool>>,
) -> Result<HttpResponse, Error> {
    match get_token(&req) {
        Ok(token) => {
            println!("Failed to decode token: {}", token);
            let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            match decode_token(&token, &secret) {
                Ok(claims) => {
                    println!("Decoded claims: {:?}", claims);
                    if claims.purpose != "create_user" {
                        return Err(ErrorUnauthorized("Invalid token "));
                    }
                }
                Err(e) => {
                    println!("Failed to decode token: {}", e);
                    return Err(ErrorUnauthorized("Invalid token"));
                }
            }
            Ok(HttpResponse::Ok().json("User created successfully"))
        }
        Err(e) => {
            println!("Failed to extract token: {}", e);
            Err(ErrorUnauthorized("Invalid token"))
        }
    }
}
