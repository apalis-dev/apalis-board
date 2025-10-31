use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::Outlet;

use crate::components::sidebar::Sidebar;

#[component]
pub fn Layout() -> impl IntoView {
    let formatter = |text| format!("{text} — Apalis Board");
    view! {
        <div class="grid grid-cols-[14rem_1fr]">
            <Title formatter />
            <Sidebar />
            <main class="flex flex-col h-screen">
                <Outlet />
            </main>
        </div>
    }
}
