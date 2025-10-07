use crate::components::authentication::database::Database;
use crate::components::authentication::models::{Claims, LoginOtp};
use crate::components::db::AsyncConnectionPool;
use crate::components::utils::user_authentication::generate_encrypted_token::generate_encrypted_token;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, error::ErrorUnauthorized, web};
use serde_json::json;
use std::sync::Arc;

pub async fn login_otp_validation(
    req: HttpRequest,
    payload: web::Json<LoginOtp>,
    pool: web::Data<Arc<AsyncConnectionPool>>,
) -> Result<HttpResponse, Error> {
    let secret = std::env::var("KEY").expect("KEY must be set");

    let mut conn = match pool.get().await {
        Some(c) => c,
        None => return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available")),
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
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => {
            drop(tx);
            pool.return_connection(conn).await;
            println!("No claims found in extensions");
            return Err(ErrorUnauthorized(json!({
                "success": false,
                "message": "Missing claims"
            })));
        }
    };

    if claims.purpose != "login_token" {
        drop(tx);
        pool.return_connection(conn).await;
        return Err(ErrorUnauthorized("Invalid Credentials 001"));
    }

    if let Err(_) = Database::compare_otp(&claims.sub, &payload.otp, &pool).await {
        drop(tx);
        pool.return_connection(conn).await;
        return Err(actix_web::error::ErrorInternalServerError(json!({
            "message": "OTP validation failed",
            "success": false
        })));
    }

    let user_uuid = match claims.user_uuid {
        Some(id) => id,
        None => {
            drop(tx);
            pool.return_connection(conn).await;
            return Err(actix_web::error::ErrorUnauthorized(
                "Missing UUID in token claims",
            ));
        }
    };

    let session_id = match Database::create_new_session(user_uuid, None, &mut tx).await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to create session: {}", e);
            drop(tx);
            pool.return_connection(conn).await;
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
            Some(user_uuid),
        ),
        generate_encrypted_token(
            &claims.sub,
            &secret,
            "refresh_token",
            21600,
            Some(session_id),
            Some(user_uuid),
        ),
    ) {
        (Ok(at), Ok(rt)) => (at, rt),
        _ => {
            drop(tx);
            pool.return_connection(conn).await;
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "success": false,
                "message": "Token generation failed"
            })));
        }
    };

    if let Err(e) = tx.commit().await {
        pool.return_connection(conn).await;
        return Err(actix_web::error::ErrorInternalServerError(json!({
            "success": false,
            "message": "Failed to commit transaction",
            "error": e.to_string()
        })));
    }

    pool.return_connection(conn).await;

    let response = json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "expires_in": 15,
        "success": true
    });

    Ok(HttpResponse::Ok().json(response))
}
