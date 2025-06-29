use crate::components::api_hook::authentication_api::AuthenticationApi;
use crate::components::function_hook::parse_api_response::parse_api_response;
use crate::components::types::auth::{OtpForm, OtpValidateApi, SignUpForm};
use crate::context::loading::use_loading;
use gloo::console::log;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub form_state: UseStateHandle<SignUpForm>,
}
#[function_component(Otp)]
pub fn otp(props: &LoginProps) -> Html {
    let loading = &use_loading();
    let form_state_otp: UseStateHandle<OtpForm> = use_state(|| OtpForm { otp: "".into() });
    let email = props.form_state.email.clone();
    let validate_otp: Callback<String> = {
        let loading = loading.clone();
        let email = email.clone();
        Callback::from(move |otp_value: String| {
            if !otp_value.is_empty() && !email.is_empty() {
                let form_data = OtpValidateApi {
                    otp: otp_value.clone(),
                    email: email.clone(),
                };
                let loading = loading.clone();
                spawn_local(async move {
                    loading.set_loading.emit(true);
                    let response = AuthenticationApi::validate_opt(&form_data).await;
                    match response {
                        Ok(resp) => match resp.text().await {
                            Ok(body_text) => match parse_api_response(&body_text) {
                                Ok(api_resp) => {
                                    log!("✅ OTP API success:", api_resp.success);
                                    loading.set_loading.emit(false);
                                }
                                Err(err) => {
                                    log!("❌ Failed to parse response:", format!("{:?}", err));
                                    loading.set_loading.emit(false);
                                }
                            },
                            Err(err) => {
                                log!("❌ Failed to read response text:", format!("{:?}", err));
                                loading.set_loading.emit(false);
                            }
                        },
                        Err(err) => {
                            log!("❌ OTP validation request failed:", format!("{:?}", err));
                            loading.set_loading.emit(false);
                        }
                    }
                    loading.set_loading.emit(false);
                });
            } else {
                log!("⚠️ OTP or email is empty, skipping validation");
                loading.set_loading.emit(false);
            }
        })
    };

    let on_input = {
        let form_state = form_state_otp.clone();
        let validate_otp = validate_otp.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let name = input.name();
                let mut value = input.value();
                if name == "otp" {
                    if value.len() > 6 {
                        value.truncate(6);
                        input.set_value(&value);
                    }
                    let mut new_state = (*form_state).clone();
                    new_state.otp = value.clone();
                    form_state.set(new_state);
                    if value.len() == 6 {
                        validate_otp.emit(value);
                    }
                }
            }
        })
    };
    html! {
     <>
      <h2>{ "Verify OTP" }</h2>
        <div class="form-group">
                <label for="otp">{ "OTP" }</label>
                <input
                    type="number"
                    id="otp"
                    name="otp"
                    placeholder="XXXXXX"
                    required=true
                    value={form_state_otp.otp.clone()}
                    class="default-textfield"
                    oninput={on_input}
                />
        </div>
        <div class="form-footer">
             <span>{ "Didn't receive the OTP?" }</span>
             <a href="#" class="signup-link">{ "Resend OTP" }</a>
         </div>
    </>
    }
}
