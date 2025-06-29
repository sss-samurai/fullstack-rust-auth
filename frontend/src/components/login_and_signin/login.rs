use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub toggle_form: Callback<MouseEvent>,
}

#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    html! {
        <>
            <h2>{ "Login" }</h2>
            <div class="form-group">
                <label for="email">{ "Email" }</label>
                <input type="email" id="email" name="email" placeholder="Enter your email" required=true class="default-textfield"/>
            </div>
            <div class="form-group">
                <label for="password">{ "Password" }</label>
                <input type="password" id="password" name="password" placeholder="Enter your password" required=true class="default-textfield"/>
            </div>
            <button type="submit" class="login-button">{ "Login" }</button>
            <div class="form-footer">
                <span>{ "Don't have an account?" }</span>
                <a href="#" class="signup-link"
                    onclick={props.toggle_form.clone()}
                >{ "Sign up" }</a>
            </div>
        </>
    }
}
