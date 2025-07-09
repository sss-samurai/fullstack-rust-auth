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
}
#[derive(Clone, PartialEq)]
pub struct PasswordForm {
    pub _password: String,
    pub password: String,
}
#[derive(Clone, PartialEq, Serialize)]
pub struct OtpValidateApi {
    pub otp: String,
    pub email: String,
}
