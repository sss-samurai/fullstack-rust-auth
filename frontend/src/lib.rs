mod components;
mod context;
use components::MainTemplate;
use crate::context::loading_provider::LoadingProvider;
use yew_router::prelude::*;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <LoadingProvider>
                <MainTemplate />
            </LoadingProvider>
        </BrowserRouter>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
