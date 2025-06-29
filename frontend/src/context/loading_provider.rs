use yew::prelude::*;
use crate::context::loading::LoadingState;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}
#[function_component(LoadingProvider)]
pub fn loading_provider(props: &Props) -> Html {
    let is_loading = use_state(|| false);
    let set_loading = {
        let is_loading = is_loading.clone();
        Callback::from(move |val: bool| is_loading.set(val))
    };
    let state: LoadingState = LoadingState {
        is_loading: *is_loading,
        set_loading,
    };
    html! {
        <ContextProvider<LoadingState> context={state}>
            { for props.children.iter() }
        </ContextProvider<LoadingState>>
    }
}
