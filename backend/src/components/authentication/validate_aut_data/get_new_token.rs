use crate::components::authentication::models::Claims;
use crate::components::utils::user_authentication::generate_encrypted_token::generate_encrypted_token;
use actix_web::{Error, HttpMessage};
use actix_web::{HttpRequest, HttpResponse, error::ErrorUnauthorized};
use serde_json::json;
pub async fn get_new_token(req: HttpRequest) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();

    if let Some(claims) = extensions.get::<Claims>() {
        println!("Got claims: {:?}", claims);

        if claims.purpose != "refresh_token" {
            return Err(ErrorUnauthorized("Invalid Credentials 001"));
        }
        let secret = std::env::var("KEY").expect("KEY must be set");
        match (
            generate_encrypted_token(&claims.sub, &secret, "access_token", 3600, None),
            generate_encrypted_token(&claims.sub, &secret, "refresh_token", 2592000, None),
        ) {
            (Ok(access_token), Ok(refresh_token)) => {
                let response = json!({
                    "access_token": access_token,
                    "refresh_token": refresh_token,
                    "expires_in": 3600,
                    "success": true
                });
                Ok(HttpResponse::Ok().json(response))
            }
            _ => Err(actix_web::error::ErrorInternalServerError(json!({
                "success": false,
                "message": "Token generation failed"
            }))),
        }
    } else {
        Err(ErrorUnauthorized(json!({
            "success": false,
            "message": "Missing claims"
        })))
    }
}
