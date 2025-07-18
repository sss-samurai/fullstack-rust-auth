use crate::components::authentication::database::Database;
use crate::components::authentication::models::PasswordClaims;
use crate::components::db::AsyncConnectionPool;
use crate::components::utils::user_authentication::generate_encrypted_token::generate_encrypted_token;
use crate::components::utils::user_authentication::is_valid_password::is_valid_password;
use crate::components::{
    authentication::models::Claims, utils::user_authentication::password_utils::PasswordUtils,
};
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, error::ErrorUnauthorized, web};
use serde_json::json;
use std::sync::Arc;

pub async fn create_new_user(
    req: HttpRequest,
    payload: web::Json<PasswordClaims>,
    pool: web::Data<Arc<AsyncConnectionPool>>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();

    if let Some(claims) = extensions.get::<Claims>() {
        println!("Got claims: {:?}", claims);

        if claims.purpose != "create_password" {
            return Err(ErrorUnauthorized("Invalid purpose for creating a new user"));
        }

        if !is_valid_password(&payload.password) {
            return Err(ErrorUnauthorized("Invalid password"));
        }

        match PasswordUtils::hash_password(&payload.password) {
            Ok(hashed_password) => {
                match Database::create_new_user(&claims.sub, &hashed_password, &pool).await {
                    Ok(_) => {
                        let secret = std::env::var("KEY").expect("KEY must be set");

                        match (
                            generate_encrypted_token(&claims.sub, &secret, "access_token", 3600),
                            generate_encrypted_token(&claims.sub, &secret, "id_token", 3600),
                            generate_encrypted_token(
                                &claims.sub,
                                &secret,
                                "refresh_token",
                                2592000,
                            ),
                        ) {
                            (Ok(access_token), Ok(id_token), Ok(refresh_token)) => {
                                let response = json!({
                                    "access_token": access_token,
                                    "id_token": id_token,
                                    "refresh_token": refresh_token,
                                    "expires_in": 3600
                                });
                                Ok(HttpResponse::Ok().json(response))
                            }
                            _ => Err(actix_web::error::ErrorInternalServerError(json!({
                                "success": false,
                                "message": "Token generation failed"
                            }))),
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to create user: {}", e);
                        Err(actix_web::error::ErrorInternalServerError(json!({
                            "success": false,
                            "message": "Failed to create user"
                        })))
                    }
                }
            }
            Err(e) => Err(ErrorUnauthorized(format!("Password hashing failed: {}", e))),
        }
    } else {
        println!("No claims found in extensions");
        Err(ErrorUnauthorized(json!({
            "success": false,
            "message": "Missing claims"
        })))
    }
}
