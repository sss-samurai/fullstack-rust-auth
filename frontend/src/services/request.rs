use gloo::console::log;
use gloo_net::http::{Request, Response};
use serde::Serialize;

use crate::components::config::api_path::ApiPath;

pub struct RequestApi;

impl RequestApi {
    pub async fn post<T: Serialize>(
        uri: &str,
        body: &T,
        token: Option<&str>,
    ) -> Result<Response, gloo_net::Error> {
        let base_url = ApiPath::get_api_base_url();
        let full_url = format!("{}{}", base_url, uri);

        let mut req = Request::post(&full_url).header("Content-Type", "application/json");
            log!("✅ Token received: {:?}", token);

        if let Some(t) = token {
            log!("✅ Token received: {:?}", t);
            req = req.header("Authorization", &format!("Bearer {}", t));
        }

        req.json(body)?.send().await
    }

    pub async fn put<T: Serialize>(
        uri: &str,
        body: &T,
        token: Option<&str>,
    ) -> Result<Response, gloo_net::Error> {
        let base_url = ApiPath::get_api_base_url();
        let full_url = format!("{}{}", base_url, uri);
        let mut req = Request::put(&full_url).header("Content-Type", "application/json");

        if let Some(t) = token {
            req = req.header("Authorization", &format!("Bearer {}", t));
        }

        req.json(body)?.send().await
    }

    pub async fn get(uri: &str, token: Option<&str>) -> Result<Response, gloo_net::Error> {
        let base_url = ApiPath::get_api_base_url();
        let full_url = format!("{}{}", base_url, uri);
        let mut req = Request::get(&full_url);

        if let Some(t) = token {
            req = req.header("Authorization", &format!("Bearer {}", t));
        }

        req.send().await
    }

    pub async fn delete(uri: &str, token: Option<&str>) -> Result<Response, gloo_net::Error> {
        let base_url = ApiPath::get_api_base_url();
        let full_url = format!("{}{}", base_url, uri);

        let mut req = Request::delete(&full_url);

        if let Some(t) = token {
            req = req.header("Authorization", &format!("Bearer {}", t));
        }

        req.send().await
    }
}
