use crate::components::router::main_router::MainRouter;
use crate::components::sidebar::Sidebar;
use crate::components::utils::loading::loding_default::LodingDefault;
use crate::context::loading::use_loading;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
#[function_component(MainTemplate)]
pub fn main_template() -> Html {
    let loading = &use_loading();
    let is_open: UseStateHandle<bool> = use_state(|| true);
    let is_dark_theme: UseStateHandle<bool> = use_state(|| true);
    let toggle_sidebar: Callback<MouseEvent> = {
        let is_open: UseStateHandle<bool> = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };
    let toggle_theme: Callback<MouseEvent> = {
        let is_dark_theme: UseStateHandle<bool> = is_dark_theme.clone();
        Callback::from(move |_| is_dark_theme.set(!*is_dark_theme))
    };
    let nav_class: &'static str = if *is_open {
        "nave-bar-open"
    } else {
        "nave-bar-close"
    };
    let header_class: &'static str = if *is_open {
        "header-open"
    } else {
        "header-close"
    };
    let body_class: &'static str = if *is_open { "body-open" } else { "body-close" };
    let theme_class: &'static str = if *is_dark_theme { "dark" } else { "light" };
    let sidebar_icon: IconId = if *is_open {
        IconId::LucideArrowLeftCircle
    } else {
        IconId::LucideArrowRightCircle
    };
    let theme_icon: IconId = if *is_dark_theme {
        IconId::LucideSun
    } else {
        IconId::LucideMoon
    };
    html! {
         <Suspense fallback={html! { <LodingDefault/> }}>
             { if loading.is_loading {
                html! { <LodingDefault/> }
            } else {
                html! {}
            }}
            <div class={theme_class}>
                <div class={nav_class}>
                    <Sidebar is_open={*is_open} />
                </div>
                <div class={header_class}>
                    <div class="header-body">
                        <button onclick={toggle_sidebar} class="icon-button">
                            <Icon icon_id={sidebar_icon} class="medium-icon" />
                        </button>
                    </div>
                    <div class="header-body">
                        <button onclick={toggle_theme} class="icon-button">
                            <Icon icon_id={theme_icon} class="medium-icon" />
                        </button>
                    </div>
                </div>
                <div class={body_class}>
                     <MainRouter />
                 </div>
            </div>
         </Suspense>
    }
}
