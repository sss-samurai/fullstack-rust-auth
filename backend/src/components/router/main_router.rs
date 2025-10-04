use crate::components::authentication::get_otp::get_otp;
use crate::components::authentication::validate_aut_data::create_session::login_get_otp;
use crate::components::authentication::validate_aut_data::get_new_token::get_new_token;
use crate::components::authentication::validate_aut_data::{
    create_new_user::create_new_user, validate_otp::validate_otp,
};
use crate::components::router::middleware::auth_middleware::AuthMiddleware;
use actix_web::web;

pub fn main_router(cfg: &mut web::ServiceConfig) {
    // Public routes
    cfg.route("/get-otp", web::post().to(get_otp));
    cfg.route("/validate-otp", web::post().to(validate_otp));
    cfg.route("/login-get-otp", web::post().to(login_get_otp));

    // Protected routes
    cfg.service(
        web::scope("/protected")
            .wrap(AuthMiddleware)
            .route("/create-new-user", web::post().to(create_new_user))
            .service(
                web::scope("")
                    .app_data(web::PayloadConfig::new(0))  
                    .route("/get-new-token", web::post().to(get_new_token))
            )
    );
}
