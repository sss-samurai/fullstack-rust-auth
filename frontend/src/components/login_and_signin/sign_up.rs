use crate::components::api_hook::authentication_api::AuthenticationApi;
use crate::components::function_hook::error_class::error_class;
use crate::components::function_hook::field_validation::validate_email;
use crate::components::function_hook::parse_api_response::parse_api_response;
use crate::components::login_and_signin::otp_and_password::otp::Otp;
use crate::components::types::auth::{SignUpForm, SignUpFormError};
use crate::context::loading::use_loading;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SignUpProps {
    pub toggle_form: Callback<MouseEvent>,
}
#[function_component(SignUp)]
pub fn sign_up(props: &SignUpProps) -> Html {
    let loading = &use_loading();
    let form_state: UseStateHandle<SignUpForm> = use_state(|| SignUpForm { email: "".into() });
    let dialog_type: UseStateHandle<String> = use_state(|| "SIGN_UP".into());
    let form_state_error: UseStateHandle<SignUpFormError> =
        use_state(|| SignUpFormError { email: false });
    let on_input = {
        let form_state = form_state.clone();
        let form_state_error = form_state_error.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = target {
                let name = input.name();
                let value = input.value();
                let mut new_state = (*form_state).clone();
                let mut new_state_error = (*form_state_error).clone();
                match name.as_str() {
                    "email" => new_state.email = value,
                    _ => {}
                }
                match name.as_str() {
                    "email" => new_state_error.email = false,
                    _ => {}
                }
                form_state.set(new_state);
                form_state_error.set(new_state_error);
            }
        })
    };

    let on_submit = {
        let form_state = form_state.clone();
        let form_state_error = form_state_error.clone();
        let loading = loading.clone();
        let dialog_type = dialog_type.clone(); // <-- clone here

        Callback::from(move |_e: MouseEvent| {
            let error_data = SignUpFormError {
                email: validate_email(&form_state.email),
            };
            form_state_error.set(error_data.clone());

            if !error_data.email {
                let form_data = (*form_state).clone();
                let loading = loading.clone();
                let dialog_type = dialog_type.clone();

                spawn_local(async move {
                    loading.set_loading.emit(true);
                    let response = AuthenticationApi::get_opt(&form_data).await;
                    match response {
                        Ok(resp) => match resp.text().await {
                            Ok(body_text) => match parse_api_response(&body_text) {
                                Ok(api_resp) => {
                                    if api_resp.success {
                                        dialog_type.set("ENTER_OTP".into());
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
    };
    html! {
    <form>
        {
            if dialog_type.as_str()=="ENTER_OTP" {
                html! {
                    <Otp form_state={form_state.clone()}/>
                }
            } else if dialog_type.as_str()=="SIGN_UP" {
                html! {
                    <>
                        <h2>{ "Sign Up" }</h2>
                        <div class="form-group">
                            <label for="email">{ "Email" }</label>
                            <input
                                type="email"
                                id="signup-email"
                                name="email"
                                placeholder="Enter your email"
                                class={ error_class(&form_state_error.email) }
                                required=true
                                value={form_state.email.clone()}
                                oninput={on_input.clone()}
                            />
                        </div>
                        <button type="button" class="login-button" onclick={on_submit}>
                            { "Sign Up" }
                        </button>
                        <div class="form-footer">
                            <span>{ "Already have an account?" }</span>
                            <a href="#" class="signup-link" onclick={props.toggle_form.clone()}>
                                { "Login" }
                            </a>
                        </div>
                    </>
                }
            } else {
                html! { <></> }
            }
        }
    </form>
}
}
