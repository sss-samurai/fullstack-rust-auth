// use gloo_storage::{Storage, CookieStorage};
// use gloo_storage::cookie::CookieOptions;
// use gloo::utils::window;
// use serde::Serialize;
// use gloo_net::http::{Request, Response};
// use crate::components::config::api_path::ApiPath;

// pub struct AuthenticationApi;

// impl AuthenticationApi {
//     async fn post_json(endpoint: &str, body: &impl Serialize) -> Result<Response, gloo_net::Error> {
//         let base_url = ApiPath::get_api_base_url();
//         let full_url = format!("{}{}", base_url, endpoint);

//         let access_token: String = CookieStorage::get("access_token").unwrap_or_default();

//         let mut request = Request::post(&full_url)
//             .header("Content-Type", "application/json")
//             .json(body)?;

//         if !access_token.is_empty() {
//             request = request.header("Authorization", &format!("Bearer {}", access_token));
//         }

//         let mut response = request.send().await?;

//         if response.status() == 401 {
//             let refresh_token: String = CookieStorage::get("refresh_token").unwrap_or_default();
//             if refresh_token.is_empty() {
//                 Self::redirect_to_login();
//                 return Err(gloo_net::Error::JsError("No refresh token".into()));
//             }

//             let refresh_response = Request::post(&format!("{}refresh-token", base_url))
//                 .header("Authorization", &format!("Bearer {}", refresh_token))
//                 .send()
//                 .await?;

//             if refresh_response.status() == 200 {
//                 let new_tokens: TokenResponse = refresh_response.json().await?;

//                 let options = CookieOptions::default()
//                     .with_path("/")
//                     .with_secure(true)
//                     .with_same_site(gloo_storage::cookie::SameSite::Lax);

//                 CookieStorage::set_with_options("access_token", &new_tokens.access_token, &options)?;
//                 CookieStorage::set_with_options("refresh_token", &new_tokens.refresh_token, &options)?;

//                 let retry_request = Request::post(&full_url)
//                     .header("Content-Type", "application/json")
//                     .header("Authorization", &format!("Bearer {}", new_tokens.access_token))
//                     .json(body)?;

//                 response = retry_request.send().await?;
//             } else {
//                 Self::redirect_to_login();
//                 return Err(gloo_net::Error::JsError("Refresh token expired".into()));
//             }
//         }

//         Ok(response)
//     }

//     fn redirect_to_login() {
//         CookieStorage::delete("access_token");
//         CookieStorage::delete("refresh_token");

//         let location = window().location();
//         let _ = location.set_href("/login");
//     }
// }
