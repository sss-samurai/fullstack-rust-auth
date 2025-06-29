use crate::components::login_and_signin::login::Login;
use crate::components::login_and_signin::sign_up::SignUp;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
enum AuthPage {
    Login,
    SignUp,
}

#[function_component(MainLoginAndSignin)]
pub fn main_login_and_signin() -> Html {
    let page = use_state(|| AuthPage::Login);

    let toggle_form = {
        let page = page.clone();
        Callback::from(move |_| {
            page.set(match *page {
                AuthPage::Login => AuthPage::SignUp,
                AuthPage::SignUp => AuthPage::Login,
            })
        })
    };
    let container_class = match *page {
        AuthPage::Login => "login-container",
        AuthPage::SignUp => "login-container signup-active",
    };
    let form_class = match *page {
        AuthPage::Login => "login-form",
        AuthPage::SignUp => "login-form signup-active",
    };
    html! {
        <div class={container_class}>
            <form class={form_class}>
                {
                    if *page == AuthPage::Login {
                        html! { <Login toggle_form={toggle_form.clone()} /> }
                    } else {
                        Html::default()
                    }
                }
                {
                    if *page == AuthPage::SignUp {
                        html! { <SignUp toggle_form={toggle_form.clone()} /> }
                    } else {
                        Html::default()
                    }
                }
            </form>
        </div>
    }
}