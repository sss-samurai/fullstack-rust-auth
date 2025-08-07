use wasm_bindgen::JsCast;
use web_sys::{window, HtmlDocument};

pub struct CookieManager;

impl CookieManager {
    fn html_document() -> HtmlDocument {
        window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window")
            .dyn_into::<HtmlDocument>()
            .expect("document should be an HtmlDocument")
    }

    pub fn set(name: &str, value: &str, days: i32) {
        let expires = js_sys::Date::new_0().get_time()
            + (days as f64) * 24.0 * 60.0 * 60.0 * 1000.0;
        let expires_str =
            js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(expires)).to_utc_string();

        let cookie_str = format!("{name}={value}; expires={expires_str}; path=/");
        Self::html_document()
            .set_cookie(&cookie_str)
            .expect("Failed to set cookie");
    }

    pub fn get(name: &str) -> Option<String> {
        let cookies = Self::html_document()
            .cookie()
            .expect("Failed to get cookies");

        for cookie in cookies.split(';') {
            let cookie = cookie.trim();
            if let Some((k, v)) = cookie.split_once('=') {
                if k == name {
                    return Some(v.to_string());
                }
            }
        }
        None
    }
    

    pub fn delete(name: &str) {
        let cookie = format!("{name}=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/");
        Self::html_document()
            .set_cookie(&cookie)
            .expect("Failed to delete cookie");
    }
}
