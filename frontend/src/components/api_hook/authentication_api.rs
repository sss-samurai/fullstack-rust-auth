use crate::components::config::api_path::ApiPath;
use gloo::console::log;
use gloo_net::http::{Request, Response};
use serde::Serialize;
pub struct AuthenticationApi;
impl AuthenticationApi {
    pub async fn get_opt(body: &impl Serialize) -> Result<Response, gloo_net::Error> {
        Self::post_json("get_otp", body).await
    }

    pub async fn validate_opt(body: &impl Serialize) -> Result<Response, gloo_net::Error> {
        Self::post_json("validate_otp", body).await
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
