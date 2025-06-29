use yew::prelude::*;
use yew_icons::{Icon, IconId};
use gloo::console::log;
use yew_router::prelude::*;
use crate::components::routes::Route;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub is_open: bool,
}

struct SidebarItem {
    name: &'static str,
    icon: IconId,
    route: Route,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
        let navigator = use_navigator().unwrap(); // get router navigator

    let items = vec![
        SidebarItem {
            name: "Home",
            icon: IconId::LucideHome,
            route: Route::Home,
        },
        SidebarItem {
            name: "Settings",
            icon: IconId::LucideSettings,
            route: Route::Settings,
        },
        SidebarItem {
            name: "Profile",
            icon: IconId::LucideUser,
            route: Route::Profile,
        },
    ];

    html! {
        <>
            {
                if props.is_open {
                    html! {
                       <div class="sidebar-body">
                            {
                                for items.iter().map(|item| {
                                    let name = item.name;
                                    let icon = item.icon.clone();
                                    let route = item.route.clone();
                                    let navigator = navigator.clone();

                                    let onclick = Callback::from(move |_| {
                                        log!(format!("Navigating to: {}", name));
                                        navigator.push(&route);
                                    });

                                    html! {
                                        <button class="slide-button" title={name} {onclick}>
                                            <Icon icon_id={icon} />
                                            <span>{ name }</span>
                                        </button>
                                    }
                                })
                            }
                        </div>
                    }
                } else {
                    html! { <div>{"Sidebar is Closed"}</div> }
                }
            }
        </>
    }
}
