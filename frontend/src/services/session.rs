use crate::components::types::auth::HttpMethod;
use crate::services::request::RequestApi;
use gloo_net::http::Response;
use serde::Serialize;

pub async fn manage_api<T: Serialize>(
    method: HttpMethod,
    uri: &str,
    body: Option<&T>,
    token: Option<&str>,
) -> Result<Response, gloo_net::Error> {
    let response = match method {
        HttpMethod::GET => RequestApi::get(uri, token).await,
        HttpMethod::POST => {
            let body = body.expect("POST requires body");
            RequestApi::post(uri, body, token).await
        }
        HttpMethod::PUT => {
            let body = body.expect("PUT requires body");
            RequestApi::put(uri, body, token).await
        }
        HttpMethod::DELETE => RequestApi::delete(uri, token).await,
    }?;

    if response.status() != 401 {
        return Ok(response);
    }



    Ok(response)
}
