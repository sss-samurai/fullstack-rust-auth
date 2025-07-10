// File: src/context/snackbar.rs
use gloo::timers::future::sleep;
use std::time::Duration;
use yew::prelude::*;

use crate::snack_bar_context::snack_bar::SnackbarType;

#[derive(Clone, PartialEq)]
pub struct SnackbarContext {
    pub show: Callback<(String, SnackbarType)>,
}

#[derive(Properties, PartialEq)]
pub struct SnackbarProviderProps {
    pub children: Children,
}

#[function_component(SnackbarProvider)]
pub fn snackbar_provider(props: &SnackbarProviderProps) -> Html {
    let message = use_state(|| "".to_string());
    let visible = use_state(|| false);
    let snackbar_type = use_state(|| SnackbarType::Success); // default

    let show = {
        let message = message.clone();
        let visible = visible.clone();
        let snackbar_type = snackbar_type.clone();
        Callback::from(move |(msg, kind): (String, SnackbarType)| {
            message.set(msg);
            snackbar_type.set(kind.clone());
            visible.set(true);

            let visible = visible.clone();
            wasm_bindgen_futures::spawn_local(async move {
                sleep(Duration::from_secs(3)).await;
                visible.set(false);
            });
        })
    };

    let context = SnackbarContext { show };

    let class = match *snackbar_type {
        SnackbarType::Success => "snackbar success",
        SnackbarType::Warning => "snackbar warning",
        SnackbarType::Error => "snackbar error",
    };

    html! {
        <ContextProvider<SnackbarContext> context={context}>
            { for props.children.iter() }
            if *visible {
                <div class={class}>
                    { (*message).clone() }
                </div>
            }
        </ContextProvider<SnackbarContext>>
    }
}
