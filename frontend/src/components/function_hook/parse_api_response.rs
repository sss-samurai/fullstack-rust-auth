use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub message: String,
    pub success: bool,
}

/// Parses the JSON body and returns a typed result
pub fn parse_api_response(body_text: &str) -> Result<ApiResponse, serde_json::Error> {
    serde_json::from_str::<ApiResponse>(body_text)
}
