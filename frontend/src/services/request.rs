use gloo_net::http::{Request, Response};
use serde::Serialize;

pub struct RequestApi;

impl RequestApi {
    pub async fn post<T: Serialize>(
        uri: &str,
        body: &T,
        token: Option<&str>,
    ) -> Result<Response, gloo_net::Error> {
        let mut req = Request::post(uri).header("Content-Type", "application/json");

        if let Some(t) = token {
            req = req.header("Authorization", &format!("Bearer {}", t));
        }

        req.json(body)?.send().await
    }

    pub async fn put<T: Serialize>(
        uri: &str,
        body: &T,
        token: Option<&str>,
    ) -> Result<Response, gloo_net::Error> {
        let mut req = Request::put(uri).header("Content-Type", "application/json");

        if let Some(t) = token {
            req = req.header("Authorization", &format!("Bearer {}", t));
        }

        req.json(body)?.send().await
    }

    pub async fn get(uri: &str, token: Option<&str>) -> Result<Response, gloo_net::Error> {
        let mut req = Request::get(uri);

        if let Some(t) = token {
            req = req.header("Authorization", &format!("Bearer {}", t));
        }

        req.send().await
    }

    pub async fn delete(uri: &str, token: Option<&str>) -> Result<Response, gloo_net::Error> {
        let mut req = Request::delete(uri);

        if let Some(t) = token {
            req = req.header("Authorization", &format!("Bearer {}", t));
        }

        req.send().await
    }
}
