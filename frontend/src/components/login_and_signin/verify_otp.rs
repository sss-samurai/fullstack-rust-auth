use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub toggle_form: Callback<MouseEvent>,
}

#[function_component(VerifyOtp)]
pub fn verify_otp(props: &LoginProps) -> Html {
    html! {
        <>
            <h2>{ "Verify OTP" }</h2>
            <div class="form-group">
                <label for="otp">{ "OTP" }</label>
                <input type="text" id="otp" name="otp" placeholder="Enter the OTP" required=true class="default-textfield"/>
            </div>
            <button type="submit" class="login-button">{ "Verify OTP" }</button>
            <div class="form-footer">
                <span>{ "Didn't receive the OTP?" }</span>
                <a href="#" class="signup-link" onclick={props.toggle_form.clone()}>{ "Resend OTP" }</a>
            </div>
        </>
    }
}
