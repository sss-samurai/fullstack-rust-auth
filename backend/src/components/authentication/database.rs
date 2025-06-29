use crate::components::authentication::models::EmailPayload;
use crate::components::db::AsyncConnectionPool;
use actix_web::{Error, HttpResponse};
pub struct Database;

impl Database {
    pub async fn save_otp(
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
        let result = conn
            .client
            .execute(
                "INSERT INTO task_backend.email_otps (email, otp) VALUES ($1, $2)",
                &[&data.email, &data.otp],
            )
            .await;

        pool.return_connection(conn).await;
        match result {
            Ok(_) => Ok(HttpResponse::Ok().body("OTP saved")),
            Err(e) => {
                eprintln!("Failed to insert OTP into DB: {}", e);
                Err(actix_web::error::ErrorInternalServerError(
                    "DB insert failed",
                ))
            }
        }
    }
    pub async fn compare_otp(
        data: EmailPayload,
        pool: &AsyncConnectionPool,
    ) -> Result<HttpResponse, actix_web::Error> {
        let Some(conn) = pool.get().await else {
            return Ok(HttpResponse::ServiceUnavailable().body("No DB connection available"));
        };
        let result = conn
            .client
            .execute(
                "SELECT * FROM task_backend.email_otps WHERE email = $1 AND otp = $2 make it as true",
                &[&data.email, &data.otp],
            )
            .await;
        let email_check = conn
            .client
            .query_one(
                "SELECT * FROM task_backend.email_otps WHERE email = $1 LIMIT 1",
                &[&data.email],
            )
            .await;

            println!("{:?}", email_check);

        match result {
            Ok(_) => Ok(HttpResponse::Ok().body("OTP saved")),
            Err(e) => {
                eprintln!("Failed to insert OTP into DB: {}", e);
                Err(actix_web::error::ErrorInternalServerError(
                    "DB insert failed",
                ))
            }
        }
    }
}
