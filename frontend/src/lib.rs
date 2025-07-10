mod components;
mod context;
mod snack_bar_context;
use crate::context::loading_provider::LoadingProvider;
use crate::snack_bar_context::snack_bar_provider::SnackbarProvider;
use components::MainTemplate;
use yew_router::prelude::*;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <LoadingProvider>
                <SnackbarProvider>
                    <MainTemplate />
                </SnackbarProvider>
            </LoadingProvider>
        </BrowserRouter>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
