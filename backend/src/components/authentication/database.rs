use crate::components::authentication::models::EmailPayload;
use crate::components::db::AsyncConnectionPool;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{Error, HttpResponse};
use serde_json::json;
use tokio_postgres::Transaction;
use uuid::Uuid;

pub struct Database;

impl Database {
    pub async fn save_otp(
        email: &String,
        otp: &str,
        pool: &AsyncConnectionPool,
    ) -> Result<HttpResponse, Error> {
        let Some(conn) = pool.get().await else {
            return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available"));
        };
        if let Err(e) = conn
            .client
            .execute(
                "DELETE FROM auth_demo.email_otps WHERE expires_at <= NOW() OR attempt_count >= 5;",
                &[],
            )
            .await
        {
            eprintln!("Cleanup error: {:?}", e);
        }
        let exists: bool = conn
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM auth_demo.email_otps WHERE email = $1)",
                &[&email],
            )
            .await
            .map_err(|e| {
                eprintln!("DB error: {:?}", e);
                actix_web::error::ErrorInternalServerError("Database error")
            })?
            .get(0);
        let result = if exists {
            conn.client
                .execute(
                    "UPDATE auth_demo.email_otps
         SET attempt_count = $1,
             expires_at = NOW() + INTERVAL '5 minutes',
             created_at = NOW(),
             otp = $3
         WHERE email = $2",
                    &[&0, &email, &otp],
                )
                .await
        } else {
            conn.client
                .execute(
                    "INSERT INTO auth_demo.email_otps (email, otp) VALUES ($1, $2)",
                    &[&email, &otp],
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
        email: &String,
        otp: &String,
        pool: &AsyncConnectionPool,
    ) -> Result<HttpResponse, Error> {
        let Some(conn) = pool.get().await else {
            return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available"));
        };
        let _ = conn
            .client
            .execute(
                "DELETE FROM auth_demo.email_otps WHERE expires_at <= NOW() OR attempt_count >= 5;",
                &[],
            )
            .await;
        let row_opt = conn
            .client
            .query_opt(
                "SELECT otp, attempt_count FROM auth_demo.email_otps
             WHERE email = $1
             ORDER BY created_at DESC
             LIMIT 1",
                &[&email],
            )
            .await
            .map_err(|_e| actix_web::error::ErrorInternalServerError("Failed to fetch OTP"))?;
        let Some(row) = row_opt else {
            return Err(actix_web::error::ErrorBadRequest(json!({
                "message": "Somthing went wrong, please try again",
                "success": false,
            })));
        };
        let stored_otp: String = row.get("otp");
        let current_attempts: i32 = row.get("attempt_count");
        if &stored_otp == otp {
            conn.client
                .execute(
                    "DELETE FROM auth_demo.email_otps WHERE email = $1",
                    &[&email],
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
                    "UPDATE auth_demo.email_otps
                 SET attempt_count = $1
                 WHERE email = $2",
                    &[&(current_attempts + 1), &email],
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
    pub async fn save_temp_email(
        data: EmailPayload,
        pool: &AsyncConnectionPool,
    ) -> Result<HttpResponse, Error> {
        let Some(conn) = pool.get().await else {
            return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available"));
        };
        if let Err(e) = conn
            .client
            .execute(
                "DELETE FROM auth_demo.otp_audit_log WHERE expires_at <= NOW();",
                &[],
            )
            .await
        {
            eprintln!("Cleanup error: {:?}", e);
        }
        let result = conn
            .client
            .execute(
                "INSERT INTO auth_demo.otp_audit_log (email) VALUES ($1)",
                &[&data.email],
            )
            .await;
        {
            eprintln!("Error While Inserting In Audit: {:?}", result);
        }
        match result {
            Ok(_) => Ok(HttpResponse::Ok().json({
                serde_json::json!({
                    "message": "OTP saved successfully",
                    "success": true
                })
            })),
            Err(_e) => Err(actix_web::error::ErrorInternalServerError(json!({
                "message": "Some Error Occured Plz Rtry otp validation",
                "success": false,
            }))),
        }
    }
    pub async fn create_new_user<'a>(
        email: &str,
        password_hash: &str,
        tx: &Transaction<'a>,
    ) -> Result<Uuid, actix_web::Error> {
        let email_exists = tx
            .query_opt("SELECT 1 FROM auth_demo.users WHERE email = $1", &[&email])
            .await
            .map_err(|e| {
                ErrorInternalServerError(format!("Failed to check email existence: {}", e))
            })?;

        if email_exists.is_some() {
            return Err(ErrorInternalServerError(
                "User with this email already exists",
            ));
        }
        let row = tx
            .query_one(
                "INSERT INTO auth_demo.users (email, password_hash) VALUES ($1, $2) RETURNING id",
                &[&email, &password_hash],
            )
            .await
            .map_err(|e| ErrorInternalServerError(format!("DB query failed: {}", e)))?;

        let uuid_str: Uuid = row.get("id");
        Ok(uuid_str)
    }
    pub async fn create_new_session<'a>(
        user_id: Uuid,
        token_id: Option<Uuid>,
        tx: &Transaction<'a>,
    ) -> Result<Uuid, actix_web::Error> {
        if let Some(token_id) = token_id {
            tx.execute(
                "UPDATE auth_demo.refresh_tokens SET is_active = false WHERE id = $1",
                &[&token_id],
            )
            .await
            .map_err(|e| {
                ErrorInternalServerError(format!("Failed to deactivate old token: {}", e))
            })?;
        }

        let row = tx
            .query_one(
                "INSERT INTO auth_demo.refresh_tokens (user_id) VALUES ($1) RETURNING id",
                &[&user_id],
            )
            .await
            .map_err(|e| ErrorInternalServerError(format!("Failed to create session: {}", e)))?;

        let session_id: Uuid = row.get("id");
        println!("{:?}", session_id);
        Ok(session_id)
    }
    pub async fn login_user<'a>(
    email: &str,
    tx: &Transaction<'a>,
) -> Result<(String, Uuid), Error> {
    let row = tx
        .query_opt(
            "SELECT id, password_hash FROM auth_demo.users WHERE email = $1",
            &[&email],
        )
        .await
        .map_err(|e| ErrorInternalServerError(format!("Database query failed: {}", e)))?;

    let Some(row) = row else {
        return Err(ErrorUnauthorized("Invalid email or password"));
    };

    let user_id: Uuid = row.get("id");
    let password_hash: String = row.get("password_hash");

    Ok((password_hash, user_id))
}
}
