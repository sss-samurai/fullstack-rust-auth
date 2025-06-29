pub fn error_class(is_error: &bool) -> &'static str {
    if *is_error {
        "error-textfield"
    } else {
        "default-textfield"
    }
}
