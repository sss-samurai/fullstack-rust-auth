use web_sys::window;


pub struct ApiPath;
impl ApiPath {
    pub fn get_api_base_url() -> String {
        window()
            .and_then(|w| w.get("API_BASE_URL"))
            .and_then(|val| val.as_string())
            .unwrap_or_else(|| {
                // fallback if not defined
                "http://127.0.0.1:3333/".to_string()
            })
    }
}
