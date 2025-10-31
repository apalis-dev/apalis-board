use crate::components::RelativeTimeRenderer;
use crate::components::tailwind::TailwindClassesPreset;
use crate::relative_timestamp;
use apalis_core::task::status::Status;
use leptos::component;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use leptos_struct_table::TableRow;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod index;
pub mod provider;
pub mod single;

#[derive(TableRow, Serialize, Deserialize, Clone, Debug)]
#[table(classes_provider = "TailwindClassesPreset")]
pub struct Task {
    #[table(skip)]
    pub args: Value,
    /// The task's id
    #[table(renderer = "IdCellRenderer")]
    pub task_id: String,

    /// The tasks's attempts
    pub attempt: usize,

    /// The task status
    #[table(renderer = "StatusCellRenderer")]
    pub status: Status,

    /// The time a task should be run
    #[table(renderer = "RelativeTimeRenderer")]
    pub run_at: u64,

    /// The task specific data provided by the backend
    #[table(renderer = "ContextCellRenderer")]
    pub meta: Value,

    #[table(skip)]
    pub queue: String,
}

#[allow(unused_variables)]
#[component]
fn IdCellRenderer(
    class: String,
    value: Signal<String>,
    row: RwSignal<Task>,
    index: usize,
) -> impl IntoView {
    let params = use_params_map();
    let queue = move || params.with(|p| p.get("queue"));
    view! {
        <td class=class>
            <A
                href=move || {
                    format!("/queues/{}/tasks/{}", queue().unwrap_or(row.get().queue), value.get())
                }
                attr:title=value
                attr:class="max-w-40 truncate whitespace-nowrap inline-block py-1.5 px-3 rounded-lg hover:font-bold bg-charcoal-100 text-charcoal-800 dark:bg-charcoal-700 dark:text-charcoal-200"
            >
                {value}
            </A>
        </td>
    }
}

#[allow(unused_variables)]
#[component]
fn StatusCellRenderer(
    class: String,
    value: Signal<Status>,
    row: RwSignal<Task>,
    index: usize,
) -> impl IntoView {
    let badge = move || match value.get() {
        Status::Pending => "Pending",
        Status::Queued => "Queued",
        Status::Running => "Running",
        Status::Done => "Done",
        Status::Failed => "Failed",
        Status::Killed => "Killed",
        _ => "Unknown",
    };

    view! {
        <td class=class>
            <span>{badge}</span>
        </td>
    }
}

#[component]
#[allow(unused_variables)]
fn ContextCellRenderer(
    class: String,
    value: Signal<Value>,
    row: RwSignal<Task>,
    index: usize,
) -> impl IntoView {
    let keys = [
        "queue",
        "priority",
        "lock_by",
        "lock_at",
        "max_attempts",
        "done_at",
        "last_result",
    ];
    let items = move || {
        value
            .get()
            .as_object()
            .map(|obj| {
                let mut res = {
                    obj.iter()
                        .filter_map(|(k, v)| {
                            if !keys.contains(&k.as_str()) {
                                return None;
                            }
                            MetaKey::from_key_value(k, v)
                        })
                        .collect::<Vec<_>>()
                };
                res.sort_by(|a, b| {
                    keys.iter()
                        .position(|&k| k == a.key())
                        .cmp(&keys.iter().position(|&k| k == b.key()))
                });
                res
            })
            .unwrap_or_default()
    };

    view! {
        <td class=class>
            <div class="flex flex-wrap gap-2">
                {move || items().into_iter().map(MetaKey::render).collect::<Vec<_>>()}
            </div>
        </td>
    }
}

enum MetaKey {
    Queue(String),
    Priority(usize),
    LockBy(String),
    LockAt(u64),
    MaxAttempts(usize),
    DoneAt(u64),
    LastResult(Value),
}

impl MetaKey {
    fn from_key_value(key: &str, value: &Value) -> Option<Self> {
        match key {
            "queue" => value.as_str().map(|s| MetaKey::Queue(s.to_string())),
            "priority" => value.as_u64().map(|n| MetaKey::Priority(n as usize)),
            "lock_by" => value.as_str().map(|s| MetaKey::LockBy(s.to_string())),
            "lock_at" => value.as_u64().map(MetaKey::LockAt),
            "max_attempts" => value.as_u64().map(|n| MetaKey::MaxAttempts(n as usize)),
            "done_at" => value.as_u64().map(MetaKey::DoneAt),
            "last_result" => {
                if value.is_null() {
                    None
                } else {
                    Some(MetaKey::LastResult(value.clone()))
                }
            }
            _ => None,
        }
    }
    fn key(&self) -> &str {
        match self {
            MetaKey::Queue(_) => "queue",
            MetaKey::Priority(_) => "priority",
            MetaKey::LockBy(_) => "lock_by",
            MetaKey::LockAt(_) => "lock_at",
            MetaKey::MaxAttempts(_) => "max_attempts",
            MetaKey::DoneAt(_) => "done_at",
            MetaKey::LastResult(_) => "last_result",
        }
    }

    fn render_key(&self) -> &str {
        match self {
            MetaKey::Queue(_) => "Queue",
            MetaKey::Priority(_) => "Priority",
            MetaKey::LockBy(_) => "Lock By",
            MetaKey::LockAt(_) => "Lock At",
            MetaKey::MaxAttempts(_) => "Max Attempts",
            MetaKey::DoneAt(_) => "Done At",
            MetaKey::LastResult(_) => "",
        }
    }
    fn render(self) -> impl IntoView {
        match self {
            MetaKey::Queue(ns) => view! {
                <span class="inline-flex items-center gap-2 font-mono text-xs px-2 py-1 rounded-md bg-charcoal-700">
                    <span class="w-4 h-4 flex items-center justify-center rounded bg-charcoal-900 text-white text-xxs font-bold">
                        "ns"
                    </span>
                    {ns}
                </span>
            }.into_any(),
            MetaKey::Priority(p) if p > 0 => view! {
                <div class="flex items-center gap-1 px-1 py-0.5 rounded-sm border border-charcoal-700 bg-charcoal-800 hover:bg-charcoal-700 relative">
                    <div class="py-1 px-1.5 inline-flex items-center gap-x-1 text-xs bg-gray-100 text-gray-800 rounded-md dark:bg-neutral-500/20 dark:text-neutral-400">
                        <span class="font-bold text-white mx-1">{p}</span>
                        <span class="flex-shrink-0" style:color="white">
                            <svg
                                class="h-5 w-5"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="lucide lucide-arrow-up10-icon lucide-arrow-up-1-0"
                            >
                                <path d="m3 8 4-4 4 4" />
                                <path d="M7 4v16" />
                                <path d="M17 10V4h-2" />
                                <path d="M15 10h4" />
                                <rect x="15" y="14" width="4" height="6" ry="2" />
                            </svg>
                        </span>
                    </div>
                </div>
            }.into_any(),
            MetaKey::DoneAt(ts) =>  view! {
                <div class="flex items-center gap-1 px-1 py-0.5 rounded-sm border border-charcoal-700 bg-charcoal-800 hover:bg-charcoal-700 relative">
                    <div class="has-tooltip">
                        <span class="tooltip rounded shadow-sm p-1 bg-charcoal-100 text-charcoal-700 -mt-8">
                            Done at: {ts}
                        </span>
                        <span class="flex-shrink-0" style:color="white">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-5 w-5"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="lucide lucide-alarm-clock-check-icon lucide-alarm-clock-check"
                            >
                                <circle cx="12" cy="13" r="8" />
                                <path d="M5 3 2 6" />
                                <path d="m22 6-3-3" />
                                <path d="M6.38 18.7 4 21" />
                                <path d="M17.64 18.67 20 21" />
                                <path d="m9 13 2 2 4-4" />
                            </svg>
                        </span>
                    </div>
                </div>
            }.into_any(),
            MetaKey::LastResult(res) => view! {
                <div class="flex items-center gap-1 px-1 py-0.5 rounded-sm border border-charcoal-700 bg-charcoal-800 hover:bg-charcoal-700 relative">
                    <div class="has-tooltip">
                        <span class="tooltip rounded shadow-sm p-1 bg-charcoal-100 text-charcoal-700 -mt-8">
                            Last result:
                            <pre>{serde_json::to_string_pretty(&res).unwrap_or_default()}</pre>
                        </span>
                        <span class="flex-shrink-0" style:color="white">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="w-4 h-4"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="lucide lucide-shapes-icon lucide-shapes"
                            >
                                <path d="M8.3 10a.7.7 0 0 1-.626-1.079L11.4 3a.7.7 0 0 1 1.198-.043L16.3 8.9a.7.7 0 0 1-.572 1.1Z" />
                                <rect x="3" y="14" width="7" height="7" rx="1" />
                                <circle cx="17.5" cy="17.5" r="3.5" />
                            </svg>
                        </span>
                    </div>
                </div>
            }.into_any(),
            MetaKey::LockBy(by) => view! {
                <span class="inline-flex items-center gap-2 font-mono text-xs px-2 py-1 rounded-md bg-charcoal-700">
                    <span class="w-4 h-4 flex items-center justify-center rounded bg-charcoal-900 text-white text-xxs font-bold">
                        "w"
                    </span>
                    {by}
                </span>
            }.into_any(),
            MetaKey::LockAt(at) => view! {
                <div class="flex items-center gap-1 px-1 py-0.5 rounded-sm border border-charcoal-700 bg-charcoal-800 hover:bg-charcoal-700 relative">
                    <div class="has-tooltip">
                        <span class="tooltip rounded shadow-sm p-1 bg-charcoal-100 text-charcoal-700 -mt-8">
                            Lock at: {relative_timestamp(at)}
                        </span>
                        <span class="flex-shrink-0" style:color="white">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="w-4 h-4"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="lucide lucide-lock-icon lucide-lock"
                            >
                                <rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
                                <path d="M7 11V7a5 5 0 0 1 10 0v4" />
                            </svg>
                        </span>
                    </div>
                </div>
            }.into_any(),
            _ => {
                view! { <></> };
                ().into_any()
            },
        }
    }
    fn render_full(self) -> impl IntoView {
        match self {
            MetaKey::Queue(ns) => view! {
                <span class="inline-flex items-center gap-2 font-mono text-xs px-2 py-1 rounded-md bg-charcoal-700">
                    <span class="w-4 h-4 flex items-center justify-center rounded bg-charcoal-900 text-white text-xxs font-bold">
                        "ns"
                    </span>
                    {ns}
                </span>
            }.into_any(),
            MetaKey::Priority(p) => view! {
                <div class="flex items-center gap-1 px-1 py-0.5 rounded-sm border border-charcoal-700 bg-charcoal-800 hover:bg-charcoal-700 relative">
                    <div class="px-1.5 inline-flex items-center gap-x-1 text-xs bg-gray-100 text-gray-800 rounded-md dark:bg-neutral-500/20 dark:text-neutral-400">
                        <span class="inline-flex items-center gap-2 font-mono text-xs px-2 rounded-md bg-charcoal-700">
                            {p}
                        </span>
                        <span class="flex-shrink-0" style:color="white">
                            <svg
                                class="h-5 w-5"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="lucide lucide-arrow-up10-icon lucide-arrow-up-1-0"
                            >
                                <path d="m3 8 4-4 4 4" />
                                <path d="M7 4v16" />
                                <path d="M17 10V4h-2" />
                                <path d="M15 10h4" />
                                <rect x="15" y="14" width="4" height="6" ry="2" />
                            </svg>
                        </span>
                    </div>
                </div>
            }.into_any(),
            MetaKey::DoneAt(ts) =>  view! {
                <div class="flex items-center gap-1 px-1 py-0.5 rounded-sm border border-charcoal-700 bg-charcoal-800 hover:bg-charcoal-700 relative">
                    <span class="inline-flex items-center gap-2 font-mono text-xs px-2 py-1 rounded-md bg-charcoal-700">
                        {relative_timestamp(ts)}
                    </span>
                    <span class="flex-shrink-0" style:color="white">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="lucide lucide-alarm-clock-check-icon lucide-alarm-clock-check"
                        >
                            <circle cx="12" cy="13" r="8" />
                            <path d="M5 3 2 6" />
                            <path d="m22 6-3-3" />
                            <path d="M6.38 18.7 4 21" />
                            <path d="M17.64 18.67 20 21" />
                            <path d="m9 13 2 2 4-4" />
                        </svg>
                    </span>
                </div>
            }.into_any(),
            MetaKey::LastResult(res) => view! {
                <>
                    <span class="text-sm text-charcoal-400 dark:text-charcoal-300">Result</span>
                    <div class="flex w-full gap-1 px-1 py-0.5 rounded-sm border border-charcoal-700 bg-charcoal-800 relative w-full mt-2">
                        <span class="inline-flex items-center gap-2 font-mono text-xs px-2 rounded-md bg-charcoal-800 overflow-auto">
                            <pre>{serde_json::to_string_pretty(&res).unwrap_or_default()}</pre>
                        </span>
                    </div>
                </>
            }.into_any(),
            MetaKey::LockBy(by) => view! {
                <span class="inline-flex items-center gap-2 font-mono text-xs px-2 py-1 rounded-md bg-charcoal-700">
                    <span class="w-4 h-4 flex items-center justify-center rounded bg-charcoal-900 text-white text-xxs font-bold">
                        "w"
                    </span>
                    {by}
                </span>
            }.into_any(),
            MetaKey::LockAt(at) => view! {
                <div class="flex items-center gap-1 px-1 py-0.5 rounded-sm border border-charcoal-700 bg-charcoal-800 hover:bg-charcoal-700 relative">
                    <span class="inline-flex items-center gap-2 font-mono text-xs px-2 py-1 rounded-md bg-charcoal-700">
                        {relative_timestamp(at)}
                    </span>
                    <span class="flex-shrink-0" style:color="white">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="w-4 h-4"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="lucide lucide-lock-icon lucide-lock"
                        >
                            <rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
                            <path d="M7 11V7a5 5 0 0 1 10 0v4" />
                        </svg>
                    </span>
                </div>
            }.into_any(),

            MetaKey::MaxAttempts(attempts) => {
                view! {
                    <span class="inline-flex items-center gap-2 font-mono text-xs px-2 py-1 rounded-md bg-charcoal-700 min-w-8">
                        {attempts}
                    </span>
                }.into_any()
            },
        }
    }
}
