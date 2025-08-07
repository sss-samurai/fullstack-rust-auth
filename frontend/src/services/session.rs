use crate::components::types::auth::HttpMethod;
use crate::components::utils::auth::manage_cookie::CookieManager;
use crate::services::request::RequestApi;
use gloo::utils::window;
use gloo_net::http::Request;
use gloo_net::http::Response;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    refresh_token: String,
}
pub async fn manage_api<T: Serialize>(
    method: HttpMethod,
    uri: &str,
    body: Option<&T>,
    token: Option<&str>,
) -> Result<Response, gloo_net::Error> {
    let response = match method {
        HttpMethod::GET => RequestApi::get(uri, token).await,
        HttpMethod::POST => RequestApi::post(uri, body.expect("POST requires body"), token).await,
        HttpMethod::PUT => RequestApi::put(uri, body.expect("PUT requires body"), token).await,
        HttpMethod::DELETE => RequestApi::delete(uri, token).await,
    }?;
    if response.status() != 401 {
        return Ok(response);
    }
    let refresh_t = CookieManager::get("refresh_t");
    if refresh_t.is_none() {
        CookieManager::delete("acces_t");
        let _ = window().location().set_href("/login");
        return Ok(response);
    }
    let mut req =
        Request::put("protected/get-new-token").header("Content-Type", "application/json");
    if let Some(t) = refresh_t {
        req = req.header("Authorization", &format!("Bearer {}", t));
    }
    let refresh_response = req.send().await?;
    if refresh_response.status() != 200 {
        CookieManager::delete("refresh_t");
        CookieManager::delete("acces_t");
        let _ = window().location().set_href("/login");
        return Ok(refresh_response);
    }
    let tokens: TokenResponse = refresh_response.json().await?;
    CookieManager::set("acces_t", &tokens.access_token);
    CookieManager::set("refresh_t", &tokens.refresh_token);
    Ok(refresh_response)
}

async fn api_function<T: Serialize>(
    method: HttpMethod,
    uri: &str,
    body: Option<&T>,
    token: Option<&str>,
) -> Result<Response, gloo_net::Error> {
    let refresh_t = CookieManager::get("refresh_t");

    let response = match method {
        HttpMethod::GET => RequestApi::get(uri, token).await,
        HttpMethod::POST => RequestApi::post(uri, body.expect("POST requires body"), token).await,
        HttpMethod::PUT => RequestApi::put(uri, body.expect("PUT requires body"), token).await,
        HttpMethod::DELETE => RequestApi::delete(uri, token).await,
    }?;
    return Ok(response);
}
