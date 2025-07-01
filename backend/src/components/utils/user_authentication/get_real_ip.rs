use actix_web::HttpRequest;

pub fn get_real_ip(req: &HttpRequest) -> String {
    req.headers()
        .get("X-Forwarded-For")
        .and_then(|val| val.to_str().ok())
        .map(|ips| ips.split(',').next().unwrap_or("").trim().to_string())
        .unwrap_or_else(|| "UNKNOWN".into())
}
