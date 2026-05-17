use crate::layout::sidebar::Sidebar;
use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_router::components::Outlet;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <Stylesheet href="/home.css" />
        <div class="app">
            <Sidebar />
            <Outlet />
        </div>
    }
}
