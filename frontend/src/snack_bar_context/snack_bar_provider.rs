// File: src/context/snackbar.rs
use yew::prelude::*;
use gloo::timers::future::sleep;
use std::time::Duration;

#[derive(Clone, PartialEq)]
pub struct SnackbarContext {
    pub show: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct SnackbarProviderProps {
    pub children: Children,
}

#[function_component(SnackbarProvider)]
pub fn snackbar_provider(props: &SnackbarProviderProps) -> Html {
    let message = use_state(|| "".to_string());
    let visible = use_state(|| false);

    let show = {
        let message = message.clone();
        let visible = visible.clone();
        Callback::from(move |msg: String| {
            message.set(msg);
            visible.set(true);

            let visible = visible.clone();
            wasm_bindgen_futures::spawn_local(async move {
                sleep(Duration::from_secs(3)).await;
                visible.set(false);
            });
        })
    };

    let context = SnackbarContext { show };

    html! {
        <ContextProvider<SnackbarContext> context={context}>
            { for props.children.iter() }
            if *visible {
                <div class="snackbar">
                    { (*message).clone() }
                </div>
            }
        </ContextProvider<SnackbarContext>>
    }
}
