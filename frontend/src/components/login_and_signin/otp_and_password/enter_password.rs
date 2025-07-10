use crate::components::function_hook::trigger_snack::trigger_snack;
use crate::components::utils::auth::is_valid_password::is_valid_password;
use crate::{
    components::types::auth::PasswordForm,
    snack_bar_context::snack_bar::{SnackbarContext, SnackbarType},
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub temp_token: UseStateHandle<String>,
}

#[function_component(EnterPassword)]
pub fn enter_password(props: &LoginProps) -> Html {
    let temp_token = props.temp_token.clone();
    let snackbar = use_context::<SnackbarContext>().expect("No SnackbarContext found");
    let form_state_pswd: UseStateHandle<PasswordForm> = use_state(|| PasswordForm {
        _password: "".into(),
        password: "".into(),
    });
    let on_input = {
        let form_state = form_state_pswd.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let name = input.name();
                let value = input.value();
                let mut new_state = (*form_state).clone();
                match name.as_str() {
                    "_password" => new_state._password = value,
                    "password" => new_state.password = value,
                    _ => {}
                }
                form_state.set(new_state);
            }
        })
    };

    let on_submit = {
        let snackbar = snackbar.clone();
        let form_state = form_state_pswd.clone();
        Callback::from(move |_: MouseEvent| {
            if form_state._password.is_empty() || form_state.password.is_empty() {
                trigger_snack(
                    &snackbar,
                    "Please enter the password",
                    SnackbarType::Warning,
                );
                return;
            }
            if form_state._password != form_state.password {
                trigger_snack(&snackbar, "Passwords do not match", SnackbarType::Error);
                return;
            }
            match is_valid_password(form_state._password.clone()) {
                true => {
                    gloo::console::log!("Password is valid and ready to be sent to the backend");
                    trigger_snack(
                        &snackbar,
                        "Password set successfully",
                        SnackbarType::Success,
                    );
                }
                false => {
                    trigger_snack(&snackbar, "Invalid password format", SnackbarType::Error);
                }
            }
        })
    };

    html! {
        <>
            <h2>{ "Password" }</h2>
            <div class="form-group">
                <label for="_password">{ "Password" }</label>
                <input
                    type="password"
                    id="_password"
                    name="_password"
                    placeholder="Enter your password"
                    required=true
                    value={form_state_pswd._password.clone()}
                    class="default-textfield"
                    oninput={&on_input.clone()}
                />
                <label for="password">{ "Re-enter Password" }</label>
                <input
                    type="password"
                    id="password"
                    name="password"
                    placeholder="Re-enter your password"
                    required=true
                    value={form_state_pswd.password.clone()}
                    class="default-textfield"
                    oninput={on_input}
                />
            </div>
                 <button type="button" class="login-button" >
                 { "Create Account" }
                 </button>
            <div class="form-footer">
                <span>{ "Didn't receive the OTP?" }</span>
                <a href="#" class="signup-link">{ "Resend OTP" }</a>
            </div>
        </>
    }
}
