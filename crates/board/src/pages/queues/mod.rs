use crate::components::sidebar::queues_icon;
use crate::components::tailwind::TailwindClassesPreset;
use apalis_core::backend::{StatType, Statistic};
use leptos::component;
use leptos::prelude::*;
use leptos_struct_table::{EventHandler, TableRow};
use serde::{Deserialize, Serialize};
use leptos_router::components::A;

pub mod index;
pub mod provider;
pub mod single;
pub mod status;

#[derive(TableRow, Serialize, Deserialize, Clone, Debug)]
#[table(sortable, classes_provider = "TailwindClassesPreset")]
pub struct Queue {
    /// Unique identifier for the queue
    #[table(renderer = "TitleRenderer")]
    pub name: String,
    /// The stats of the queue
    #[table(renderer = "StatsCellRenderer")]
    pub stats: Vec<Statistic>,
    /// Recent activity in the queue
    #[table(renderer = "ActivityCellRenderer")]
    pub activity: Vec<usize>,
    /// Workers associated with the queue
    #[table(renderer = "WorkersCellRenderer")]
    pub workers: Vec<String>,
}

#[allow(unused_variables)]
#[component]
pub fn ActivityCellRenderer(
    class: String,
    value: Signal<Vec<usize>>,
    row: RwSignal<Queue>,
    index: usize,
) -> impl IntoView {
    // Calculate the max value for scaling
    let max = move || value.get().iter().copied().max().unwrap_or(1);

    view! {
        <td class=class>
            <div class="flex h-6 w-[5.125rem] items-end gap-0.5 rounded-sm">
                {move || {
                    value
                        .get()
                        .iter()
                        .map(|&v| {
                            let percent = if max() > 0 {
                                (v as f32 / max() as f32) * 80.0 + 20.0
                            } else {
                                20.0
                            };
                            let style = format!("height: {percent}%;");
                            // Scale height between 20% and 100% of parent
                            view! {
                                <div
                                    class="w-2.5 bg-gray-600 rounded"
                                    style=style
                                    title=v.to_string()
                                ></div>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </td>
    }
}

#[allow(unused_variables)]
#[component]
pub fn TitleRenderer(
    class: String,
    value: Signal<String>,
    row: RwSignal<Queue>,
    index: usize,
) -> impl IntoView {
    view! {
        <td class=class>
            <A href=move || format!("/queues/{}", value.get()) attr:class="flex items-center gap-2">
                <div class="text-white flex items-center gap-1 m-1">
                    <div class="border border-gray-600 rounded-sm p-1 flex items-center justify-center">
                        {queues_icon()}
                    </div>
                    <span class="ms-1">{move || value.get()}</span>
                </div>
            </A>
        </td>
    }
}

#[allow(unused_variables)]
#[component]
pub fn StatsCellRenderer(
    class: String,
    value: Signal<Vec<Statistic>>,
    row: RwSignal<Queue>,
    index: usize,
) -> impl IntoView {
    view! {
        <td class=class>
            <div class="text-white flex items-center gap-1 m-1">
                <span class="ms-1">
                    {move || {
                        value
                            .get()
                            .into_iter()
                            .filter(|stat| match stat.stat_type {
                                StatType::Percentage => true,
                                _ => false,
                            })
                            .map(|stat| {
                                view! {
                                    <div class="flex items-center">
                                        <span class="font-medium">{stat.value}</span>
                                        <span class="text-xs text-gray-500 ms-1">{stat.title}</span>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </span>
            </div>
        </td>
    }
}

#[allow(unused_variables)]
#[component]
pub fn WorkersCellRenderer(
    class: String,
    value: Signal<Vec<String>>,
    row: RwSignal<Queue>,
    index: usize,
) -> impl IntoView {
    view! {
        <td class=class>
            <div class="flex -space-x-4 rtl:space-x-reverse">
                {move || {
                    let workers = value.get();
                    let first = workers
                        .first()
                        .cloned()
                        .unwrap_or_default()
                        .chars()
                        .next()
                        .unwrap_or('?')
                        .to_ascii_uppercase();
                    let remaining = workers.len().saturating_sub(1);
                    view! {
                        <div class="relative inline-flex items-center justify-center w-8 h-8 overflow-hidden bg-background-dimmed border-1 rounded-full">
                            <span class="font-medium text-gray-600 dark:text-gray-300">
                                {first}
                            </span>
                        </div>
                        {if remaining > 0 {
                            view! {
                                <a
                                    class="z-10 flex items-center justify-center w-8 h-8 text-xs font-medium text-white bg-gray-700 border-2 border-white rounded-full hover:bg-gray-600 dark:border-gray-800"
                                    href="#"
                                >
                                    {format!("+{remaining}")}
                                </a>
                            }
                                .into_any()
                        } else {
                            view! {};
                            ().into_any()
                        }}
                    }
                }}
            </div>
        </td>
    }
}

/// Custom row renderer that adds a link to the end of the row
#[allow(unused_variables, non_snake_case)]
pub fn CustomTableRowRenderer(
    // The class attribute for the row element. Generated by the classes provider.
    class: Signal<String>,
    // The row to render.
    row: RwSignal<Queue>,
    // The index of the row. Starts at 0 for the first body row.
    index: usize,
    // The selected state of the row. True, when the row is selected.
    selected: Signal<bool>,
    // Event handler callback when this row is selected
    on_select: EventHandler<web_sys::MouseEvent>,
) -> impl IntoView {
    view! {
        <tr class=class on:click=move |mouse_event| on_select.run(mouse_event)>
            {TableRow::render_row(row, index)}
            <td class="px-3 py-2 text-right">
                <a
                    href=move || format!("/queues/{}", row.get().name)
                    class="inline-block px-3 py-1 text-sm font-medium text-white bg-gray-600 rounded hover:bg-gray-700 transition-colors"
                >
                    "View"
                </a>

            </td>
        </tr>
    }
}
