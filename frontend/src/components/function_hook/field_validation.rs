

pub fn validate_email(email: &String) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return true;
    }
    let local = parts[0];
    let domain = parts[1];
    if local.is_empty() || domain.is_empty() {
        return true;
    }
    if !domain.contains('.') {
        return true;
    }
    if domain.split('.').any(|s| s.is_empty()) {
        return true;
    }
    false
}
