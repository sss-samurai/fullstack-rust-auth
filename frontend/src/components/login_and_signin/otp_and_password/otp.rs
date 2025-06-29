use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::components::api_hook::authentication_api::AuthenticationApi;
use crate::components::types::auth::{OtpForm, SignUpForm};
use crate::context::loading::use_loading;
use crate::components::function_hook::parse_api_response::parse_api_response;
#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub form_state: UseStateHandle<SignUpForm>,
}

#[function_component(Otp)]
pub fn otp(props: &LoginProps) -> Html {
        let loading = &use_loading();

    let form_state_otp: UseStateHandle<OtpForm> = use_state(|| OtpForm { otp: "".into(), email: "".into() });
     let on_input = {
        let form_state = form_state_otp.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = target {
                let name = input.name();
                let value = input.value();
                let mut new_state = (*form_state).clone();

                match name.as_str() {
                    "otp" => new_state.otp = value,
                    _ => {}
                }
                form_state.set(new_state);
            }
        })
    };
    let on_submit = {
        let form_state = form_state_otp.clone();
        let loading = loading.clone();
        // let dialog_type = dialog_type.clone(); // <-- clone here

        Callback::from(move |_e: MouseEvent| {


            if !form_state_otp.email.is_empty() && !form_state_otp.otp.is_empty() {
                let form_data = (*form_state).clone();
                let loading = loading.clone();
                // let dialog_type = dialog_type.clone();

                spawn_local(async move {
                    loading.set_loading.emit(true);
                    let response = AuthenticationApi::validate_opt(&form_data).await;
                    match response {
                        Ok(resp) => match resp.text().await {
                            Ok(body_text) => match parse_api_response(&body_text) {
                                Ok(api_resp) => {
                                    if api_resp.success {
                                        // dialog_type.set("ENTER_OTP".into());
                                        loading.set_loading.emit(false);
                                    } else {
                                        loading.set_loading.emit(false);
                                    }
                                }
                                Err(_) => {
                                    loading.set_loading.emit(false);
                                }
                            },
                            Err(_) => {
                                loading.set_loading.emit(false);
                            }
                        },
                        Err(_) => {
                            loading.set_loading.emit(false);
                        }
                    }
                });
            }
        })
    };    html! {
        <>
            <h2>{ "Verify OTP" }</h2>
            <div class="form-group">
                <label for="otp">{ "OTP" }</label>
                <input type="number" id="otp" name="otp" placeholder="XXXXXX" required=true class="default-textfield" oninput={on_input.clone()} />
            </div>
            <button type="submit" class="login-button">{ "Verify OTP" }</button>
            <div class="form-footer">
                <span>{ "Didn't receive the OTP?" }</span>
                <a href="#" class="signup-link" >{ "Resend OTP" }</a>
            </div>
        </>
    }
}
