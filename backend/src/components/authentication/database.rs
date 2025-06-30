use crate::components::authentication::models::EmailPayload;
use crate::components::db::AsyncConnectionPool;
use actix_web::{Error, HttpResponse};
use serde_json::json;
pub struct Database;

impl Database {
    pub async fn save_otp(
        data: EmailPayload,
        pool: &AsyncConnectionPool,
    ) -> Result<HttpResponse, Error> {
        let Some(conn) = pool.get().await else {
            return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available"));
        };

        // Clean expired or over-attempted OTPs
        if let Err(e) = conn
        .client
        .execute(
            "DELETE FROM task_backend.email_otps WHERE expires_at <= NOW() OR attempt_count >= 5;",
            &[],
        )
        .await
    {
        eprintln!("Cleanup error: {:?}", e);
    }

        // Check if email already has an OTP
        let exists: bool = conn
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM task_backend.email_otps WHERE email = $1)",
                &[&data.email],
            )
            .await
            .map_err(|e| {
                eprintln!("DB error: {:?}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?
            .get(0);

        // If OTP already exists, just update the attempt count to 9
        let result = if exists {
            conn.client
                .execute(
                    "UPDATE task_backend.email_otps 
         SET attempt_count = $1,
             expires_at = NOW() + INTERVAL '5 minutes',
             created_at = NOW(),
             otp = $3
         WHERE email = $2",
                    &[&0, &data.email, &data.otp],
                )
                .await
        } else {
            conn.client
                .execute(
                    "INSERT INTO task_backend.email_otps (email, otp) VALUES ($1, $2)",
                    &[&data.email, &data.otp],
                )
                .await
        };

        match result {
            Ok(_) => Ok(HttpResponse::Ok().body("OTP saved")),
            Err(e) => {
                eprintln!("Failed to save OTP: {}", e);
                Err(actix_web::error::ErrorInternalServerError(
                    "DB operation failed",
                ))
            }
        }
    }

    pub async fn compare_otp(
        data: EmailPayload,
        pool: &AsyncConnectionPool,
    ) -> Result<HttpResponse, Error> {
        let Some(conn) = pool.get().await else {
            return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available"));
        };
        let _ = conn
            .client
            .execute(
                "DELETE FROM task_backend.email_otps WHERE expires_at <= NOW() OR attempt_count >= 5;",
                &[],
            )
            .await;
        let row_opt = conn
            .client
            .query_opt(
                "SELECT otp, attempt_count FROM task_backend.email_otps 
             WHERE email = $1 
             ORDER BY created_at DESC 
             LIMIT 1",
                &[&data.email],
            )
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError("Failed to fetch OTP"))?;
        let Some(row) = row_opt else {
            return Err(actix_web::error::ErrorBadRequest(json!({
                "message": "Somthing went wrong, please try again",
                "success": false,
            })));
        };
        let stored_otp: String = row.get("otp");
        let current_attempts: i32 = row.get("attempt_count");
        if Some(stored_otp) == data.otp {
            conn.client
                .execute(
                    "DELETE FROM task_backend.email_otps WHERE email = $1",
                    &[&data.email],
                )
                .await
                .map_err(|e| {
                    eprintln!("Failed to delete OTP: {}", e);
                    actix_web::error::ErrorInternalServerError("Failed to clean up OTP")
                })?;
            return Ok(HttpResponse::Ok().json(json!({
                "message": "OTP validated successfully",
                "success": true
            })));
        } else {
            conn.client
                .execute(
                    "UPDATE task_backend.email_otps 
                 SET attempt_count = $1 
                 WHERE email = $2",
                    &[&(current_attempts + 1), &data.email],
                )
                .await
                .map_err(|e| {
                    eprintln!("Failed to increment attempt count: {}", e);
                    actix_web::error::ErrorInternalServerError("Failed to update attempt count")
                })?;
            return Err(actix_web::error::ErrorBadRequest(json!({
                "message": "Invalid OTP",
                "success": false,
            })));
        }
    }
}
