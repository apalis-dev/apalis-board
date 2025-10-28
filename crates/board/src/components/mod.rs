use leptos::prelude::*;

use crate::resolve_timestamp;

pub mod icon;
pub mod layout;
pub mod not_found;
pub mod pagination;
pub mod sidebar;
pub mod tailwind;

#[allow(unused_variables)]
#[component]
pub fn RelativeTimeRenderer<T>(
    class: String,
    value: Signal<u64>,
    row: RwSignal<T>,
    index: usize,
) -> impl IntoView {
    let relative_time = resolve_timestamp(value);
    view! { <td class=class>{relative_time}</td> }
}
