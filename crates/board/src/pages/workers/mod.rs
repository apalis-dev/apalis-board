use crate::{
    components::tailwind::TailwindClassesPreset, pages::workers::capability::ServiceInfoDisplay,
};
use leptos::component;
use leptos::prelude::*;
use leptos_struct_table::TableRow;
use serde::{Deserialize, Serialize};

pub mod capability;
pub mod index;
pub mod provider;

#[derive(TableRow, Serialize, Deserialize, Clone, Debug)]
#[table(classes_provider = "TailwindClassesPreset")]
pub struct Worker {
    /// Unique identifier for the worker
    pub id: String,
    /// Backend of the worker
    pub backend: String,
    /// Timestamp when the worker was started
    pub started_at: u64,
    /// Timestamp of the last heartbeat received from the worker
    pub last_heartbeat: u64,
    /// Service name the worker is associated with
    #[table(renderer = "ServiceCellRenderer")]
    pub layers: String,
}

// Easy cell renderer that just displays an image from an URL.
#[component]
#[allow(unused_variables)]
fn ServiceCellRenderer(
    class: String,
    value: Signal<String>,
    row: RwSignal<Worker>,
    index: usize,
) -> impl IntoView {
    view! {
        <td class=class>
            <ServiceInfoDisplay service=value />
        </td>
    }
}
