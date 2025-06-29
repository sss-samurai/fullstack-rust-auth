use serde::Serialize;

#[derive(Clone, PartialEq)]
pub struct SignUpFormError {
    pub email: bool,
}
#[derive(Clone, PartialEq, Serialize)]
pub struct SignUpForm {
    pub email: String,
}
#[derive(Clone, PartialEq, Serialize)]
pub struct OtpForm {
    pub otp: String,
    pub email: String,
}
