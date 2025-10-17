use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::sidebar::Sidebar;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="grid h-full w-full grid-rows-1 overflow-hidden">
            <div class="grid grid-cols-[14rem_1fr] overflow-hidden">
                <Sidebar />
                <main class="grid grid-rows-1 overflow-hidden">
                    <Outlet />
                </main>
            </div>
        </div>
    }
}
