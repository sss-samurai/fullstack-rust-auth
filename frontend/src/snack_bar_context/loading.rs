use yew::prelude::*;
#[derive(Clone, PartialEq)]
pub struct LoadingState {
    pub is_loading: bool,
    pub set_loading: Callback<bool>,
}
#[hook]
pub fn use_loading() -> LoadingState {
    use_context::<LoadingState>().expect("LoadingState context not found")
}
