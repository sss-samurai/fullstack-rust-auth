use crate::components::utils::user_authentication::{
    generate_encrypted_token::generate_encrypted_token, send_mail::send_mail,
};
use actix_web::{Error, HttpResponse, error::ErrorUnauthorized, web};
use rand::Rng;
use serde_json::json;
use std::sync::Arc;

use crate::components::{
    authentication::{database::Database, models::LoginPayload},
    db::AsyncConnectionPool,
    utils::user_authentication::password_utils::PasswordUtils,
};

pub async fn login_get_otp(
    payload: web::Json<LoginPayload>,
    pool: web::Data<Arc<AsyncConnectionPool>>,
) -> Result<HttpResponse, Error> {
    print!("Login get otp called with payload: {:?}", payload);
    let Some(mut conn) = pool.get().await else {
        return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available"));
    };

    let tx = match conn.client.transaction().await {
        Ok(tx) => tx,
        Err(e) => {
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "message": "Failed to start transaction",
                "error": e.to_string()
            })));
        }
    };

    let (password_hash, user_id) = match Database::login_user(&payload.email, &tx).await {
        Ok((hash, id)) => (hash, id),
        Err(e) => return Err(ErrorUnauthorized(format!("Login failed: {}", e))),
    };
    let is_valid = match PasswordUtils::verify_password(&payload.password, &password_hash) {
        Ok(valid) => valid,
        Err(_) => return Err(ErrorUnauthorized("Password verification failed")),
    };
    if !is_valid {
        return Err(ErrorUnauthorized("Invalid email or password"));
    }

    let secret = std::env::var("KEY").expect("KEY must be set");

    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100_000..1_000_000);
    send_mail(&payload.email, otp).await.map_err(|e| {
        eprintln!("Failed to send OTP email: {}", e);
        actix_web::error::ErrorInternalServerError("Email send failed")
    })?;

    Database::save_otp(&payload.email, &otp.to_string(), &pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to save OTP to database: {}", e);
            actix_web::error::ErrorInternalServerError("Database save failed")
        })?;
    let login_token =
        match generate_encrypted_token(&payload.email, &secret, "login_token", 15, Some(user_id)) {
            Ok(token) => token,
            Err(_) => {
                return Err(actix_web::error::ErrorInternalServerError(json!({
                    "success": false,
                    "message": "Token generation failed"
                })));
            }
        };
    Ok(HttpResponse::Ok().json(json!({
        "message": "OTP sent",
        "success": true,
        "token": login_token
    })))
}
