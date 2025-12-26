use leptos::prelude::*;

use crate::relative_timestamp;

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
    let relative_time = move || relative_timestamp(value.get());
    view! { <td class=class>{relative_time}</td> }
}
