use crate::components::authentication::get_otp::get_otp;
use crate::components::authentication::validate_aut_data::validate_otp::validate_otp;
use actix_web::web;

pub fn main_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/get-otp", web::post().to(get_otp));
    cfg.route("/validate-otp", web::post().to(validate_otp));
}
