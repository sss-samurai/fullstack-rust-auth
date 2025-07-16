use crate::components::authentication::models::Claims;
use crate::components::db::AsyncConnectionPool;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, error::ErrorUnauthorized, web};
use std::sync::Arc;

pub async fn create_new_user(
    req: HttpRequest,
    pool: web::Data<Arc<AsyncConnectionPool>>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();

    if let Some(claims) = extensions.get::<Claims>() {
        println!("Got claims: {:?}", claims); // ✅ This prints actual Claims
        Ok(HttpResponse::Ok().json("User created"))
    } else {
        println!("No claims found in extensions"); // ✅ More meaningful log
        Err(ErrorUnauthorized("Missing claims"))
    }
}
