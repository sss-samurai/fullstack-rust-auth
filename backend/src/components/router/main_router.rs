use crate::components::authentication::get_otp::get_otp;
use actix_web::web;

pub fn main_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/get_otp", web::post().to(get_otp));
}
