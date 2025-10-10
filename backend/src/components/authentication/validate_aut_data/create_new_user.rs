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
    let Some(mut conn) = pool.get().await else {
        return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available"));
    };

    let mut tx = match conn.client.transaction().await {
        Ok(tx) => tx,
        Err(e) => {
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "message": "Failed to start transaction",
                "error": e.to_string()
            })));
        }
    };

    let extensions = req.extensions();
    let Some(claims) = extensions.get::<Claims>() else {
        println!("No claims found in extensions");
        return Err(ErrorUnauthorized(json!({
            "success": false,
            "message": "Missing claims"
        })));
    };

    println!("Got claims: {:?}", claims);

    if claims.purpose != "create_user" {
        return Err(ErrorUnauthorized("Invalid Credentials 001"));
    }

    if !is_valid_password(&payload.password) {
        return Err(ErrorUnauthorized("Invalid password"));
    }

    let hashed_password = match PasswordUtils::hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => return Err(ErrorUnauthorized(format!("Password hashing failed: {}", e))),
    };
    println!("Hashed password: {}", hashed_password);
    let new_user_id = match Database::create_new_user(&claims.sub, &hashed_password, &mut tx).await
    {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to create user: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "success": false,
                "message": "Failed to create user"
            })));
        }
    };

    let secret = std::env::var("KEY").expect("KEY must be set");

    let session_id = match Database::create_new_session(new_user_id, None, &mut tx).await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to create session: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "success": false,
                "message": "Failed to create session"
            })));
        }
    };
    let (access_token, refresh_token) = match (
        generate_encrypted_token(
            &claims.sub,
            &secret,
            "access_token",
            15,
            Some(session_id),
            Some(new_user_id),
        ),
        generate_encrypted_token(
            &claims.sub,
            &secret,
            "refresh_token",
            21600,
            Some(session_id),
            Some(new_user_id),
        ),
    ) {
        (Ok(at), Ok(rt)) => (at, rt),
        _ => {
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "success": false,
                "message": "Token generation failed"
            })));
        }
    };

    if let Err(e) = tx.commit().await {
        return Err(actix_web::error::ErrorInternalServerError(json!({
            "success": false,
            "message": "Failed to commit transaction",
            "error": e.to_string()
        })));
    }

    let response = json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "expires_in": 15,
        "success": true
    });

    Ok(HttpResponse::Ok().json(response))
}
