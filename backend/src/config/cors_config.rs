use actix_cors::Cors;
use actix_web::http;

pub fn cors_config() -> Cors {
    Cors::default()
        .allowed_origin("http://127.0.0.1:3000")
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        .supports_credentials()
}
