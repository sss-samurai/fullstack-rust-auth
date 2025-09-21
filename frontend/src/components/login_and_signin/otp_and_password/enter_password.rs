use crate::components::{
    api_hook::authentication_api::sign_up, function_hook::parse_api_response::parse_api_response,
    utils::auth::is_valid_password::is_valid_password,
};
use crate::context::loading::use_loading;
use gloo::console::log;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize)]
pub struct PasswordPayload {
    password: String,
}

#[derive(Clone, PartialEq)]
struct Passwords {
    password: String,
    re_password: String,
}

#[function_component(EnterPassword)]
pub fn enter_password() -> Html {
    let loading = &use_loading();

    let passwords = use_state(|| Passwords {
        password: "".to_string(),
        re_password: "".to_string(),
    });

    let status = use_state(|| false);

    let on_input_pswd = {
        let form_state = passwords.clone();
        let status = status.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let name = input.name();
                let value = input.value();
                let mut new_state = (*form_state).clone();

                if name == "password" {
                    new_state.password = value.clone();
                    status.set(is_valid_password(value));
                }

                form_state.set(new_state);
            }
        })
    };

    let on_input_re_pswd = {
        let form_state = passwords.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let name = input.name();
                let value = input.value();
                let mut new_state = (*form_state).clone();

                if name == "re_password" {
                    new_state.re_password = value;
                }

                form_state.set(new_state);
            }
        })
    };

    let create_user = {
        let passwords = passwords.clone();
        let loading = loading.clone();
        let set_loading = loading.set_loading.clone(); // ✅ clone it outside

        Callback::from(move |_e: MouseEvent| {
            let passwords = passwords.clone();
            let set_loading = set_loading.clone(); // ✅ clone again for inside spawn_local

            spawn_local(async move {
                let pswd = &passwords.password;
                let re_pswd = &passwords.re_password;

                if pswd != re_pswd {
                    passwords.set(Passwords {
                        password: "".to_string(),
                        re_password: "".to_string(),
                    });
                    log!("Passwords do not match. Resetting fields.");
                    return;
                }

                let response = sign_up(&PasswordPayload {
                    password: pswd.clone(),
                })
                .await;
                set_loading.emit(true);
                match response {
                    Ok(resp) => match resp.text().await {
                        Ok(body_text) => match parse_api_response(&body_text) {
                            Ok(api_resp) => match api_resp.token {
                                Some(token) => {
                                    log!("✅ Token received: {:?}", token);
                                    set_loading.emit(false);
                                }
                                None => {
                                    set_loading.emit(false);
                                }
                            },
                            Err(err) => {
                                log!("❌ Failed to parse response:", format!("{:?}", err));
                                set_loading.emit(false);
                            }
                        },
                        Err(err) => {
                            log!("❌ Failed to read response text:", format!("{:?}", err));
                            set_loading.emit(false);
                        }
                    },
                    Err(err) => {
                        log!("❌ OTP validation request failed:", format!("{:?}", err));
                        set_loading.emit(false);
                    }
                }
            });
        })
    };

    html! {
        <>
            <h2>{ "Password" }</h2>
            <div class="form-group">
                <label for="password">{ "Password" }</label>
                <input
                    type="password"
                    id="password"
                    name="password"
                    placeholder="Enter your password"
                    required=true
                    class="default-textfield"
                    oninput={on_input_pswd}
                />

                {
                    if *status {
                        html! {
                            <>
                                <label for="re_password">{ "Re-enter Password" }</label>
                                <input
                                    type="password"
                                    id="re_password"
                                    name="re_password"
                                    placeholder="Re-enter your password"
                                    required=true
                                    class="default-textfield"
                                    oninput={on_input_re_pswd}
                                />
                                {
                                    if !passwords.re_password.is_empty() {
                                        html! {
                                            <button
                                                type="button"
                                                onclick={create_user}
                                                class="login-button"
                                            >
                                                { "Create Account" }
                                            </button>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>

            <div class="form-footer">
                <span>{ "Didn't receive the OTP?" }</span>
                <a href="#" class="signup-link">{ "Resend OTP" }</a>
            </div>
        </>
    }
}
