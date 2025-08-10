use crate::components::config::api_path::ApiPath;
use crate::components::types::auth::{HttpMethod, OtpValidateApi, SignUpForm};
use crate::services::session::manage_api;
use gloo::console::log;
use gloo_net::http::{Request, Response};
use serde::Serialize;
pub struct AuthenticationApi;
impl AuthenticationApi {
    pub async fn get_opt(body: &SignUpForm) -> Result<Response, gloo_net::Error> {
        Self::post_json("get-otp", body).await
    }

    pub async fn validate_opt(body: &OtpValidateApi) -> Result<Response, gloo_net::Error> {
        Self::post_json("validate-otp", body).await
    }

    async fn post_json(endpoint: &str, body: &impl Serialize) -> Result<Response, gloo_net::Error> {
        let base_url = ApiPath::get_api_base_url();
        let full_url = format!("{}{}", base_url, endpoint);
        let full_url_ref: &str = &full_url;
        log!("POST {}", full_url_ref);
        Request::post(full_url_ref)
            .header("Content-Type", "application/json")
            .json(body)?
            .send()
            .await
    }
}

pub async fn get_signup_otp(body: &SignUpForm) -> Result<Response, gloo_net::Error> {
    manage_api(HttpMethod::POST, "get-otp", Some(body), false).await
}

