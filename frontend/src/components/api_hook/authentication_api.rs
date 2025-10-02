use crate::components::login_and_signin::otp_and_password::enter_password::PasswordPayload;
use crate::components::types::auth::{HttpMethod, OtpValidateApi, SignUpForm};
use crate::services::session::manage_api;
use gloo_net::http::{ Response};

pub async fn get_signup_otp(body: &SignUpForm) -> Result<Response, gloo_net::Error> {
    manage_api(HttpMethod::POST, "get-otp", Some(body), false,false).await
}
pub async fn validate_signup_otp(body: &OtpValidateApi) -> Result<Response, gloo_net::Error> {
    manage_api(HttpMethod::POST, "validate-otp", Some(body), false,false).await
}
pub async fn sign_up(body: &PasswordPayload) -> Result<Response, gloo_net::Error> {
    manage_api(HttpMethod::POST, "protected/create-new-user", Some(body), true,false).await
}