use crate::{
    api::ApiClient, components::sidebar::tasks_icon, pages::tasks::MetaKey, use_sse_provider,
    RawTask,
};
use apalis_board_types::{LogEntry, LogLevel};
use apalis_core::task::status::Status;
use futures::{FutureExt, StreamExt};
use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_params_map;

#[component]
pub fn SingleTaskView(
    #[prop(into)] task: RawTask,
    logs: RwSignal<Vec<LogEntry>>,
    queue: String,
) -> impl IntoView {
    let args_json = serde_json::to_string_pretty(&task.args).unwrap_or_default();

    let keys = [
        "queue",
        "priority",
        "lock_by",
        "lock_at",
        "max_attempts",
        "done_at",
        "last_result",
    ];
    let ctx = task.parts.ctx.clone();
    let items = move || {
        ctx.as_object()
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

    let status_class = match &task.parts.status.load() {
        Status::Done => "bg-success text-charcoal-100 dark:bg-success dark:text-charcoal-100",
        Status::Failed => "bg-error text-charcoal-100 dark:bg-error dark:text-charcoal-100",
        Status::Pending => "bg-warning text-charcoal-100 dark:bg-warning dark:text-charcoal-100",
        Status::Running => "bg-pending text-charcoal-100 dark:bg-pending dark:text-charcoal-100",
        _ => "bg-charcoal-700 text-charcoal-100 dark:bg-charcoal-800 dark:text-charcoal-100",
    };

    let task_id = task.parts.task_id.unwrap().to_string();

    view! {
        <>
            <div class="w-full bg-background-bright border-b border-gray-700 flex items-center">
                <span class="m-2 rounded-none p-1.5 bg-charcoal-700 text-text-bright">
                    {tasks_icon()}
                </span>
                <h3 class="text-base font-bold m-2">{queue}</h3>
                <span class="text-gray-500">"/"</span>
                <h4 class="text-sm m-2 flex-1">{task_id.clone()}</h4>
                <div class="flex items-center gap-3 mr-4">
                    <span class=format!(
                        "inline-flex items-center px-3 py-1 rounded-none text-xs font-medium border border-charcoal-700 {}",
                        status_class,
                    )>{task.parts.status.load().to_string()}</span>
                    <span class="py-1 px-2 inline-flex items-center gap-x-1 text-xs font-medium rounded-none border border-charcoal-700">
                        "Attempt: " {task.parts.attempt.current()}
                    </span>
                </div>
            </div>

            <div class="w-full px-6 py-6">
                <div class="grid grid-cols-1 lg:grid-cols-5 gap-6">

                    <div class="lg:col-span-2 space-y-6">

                        <div class="bg-charcoal-900 rounded-none border  border-charcoal-700 dark:border-charcoal-850 p-6 space-y-4">
                            <h2 class="text-base ">"Arguments"</h2>
                            <pre class="overflow-auto border-t max-h-40 border-charcoal-700 dark:border-charcoal-850 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-600 hover:scrollbar-thumb-charcoal-500">
                                <code class="text-sm text-gray-300 font-mono">{args_json}</code>
                            </pre>
                        </div>

                        <div class="bg-charcoal-900 rounded-none border border-charcoal-700 dark:border-charcoal-850 p-6 space-y-4">
                            <h2 class="text-base ">"Context"</h2>

                            <div class="space-y-3 border-t border-charcoal-700 dark:border-charcoal-850 pt-3">
                                {items()
                                    .into_iter()
                                    .map(move |val| {
                                        view! {
                                            <div class="flex justify-between items-start py-2 border-b border-charcoal-700 dark:border-charcoal-850 last:border-b-0">
                                                <span class="font-medium text-charcoal-400 dark:text-charcoal-300">
                                                    {val.key().to_string()}
                                                </span>
                                                <span class=" text-charcoal-100 dark:text-charcoal-100 text-right max-w-md truncate">
                                                    {val.render()}
                                                </span>
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>
                    <div class="lg:col-span-3">
                        <LogViewer items=logs />
                    </div>
                </div>
            </div>
        </>
    }
}

#[component]
pub fn TaskPage() -> impl IntoView {
    let params = use_params_map().get_untracked();
    let task_id = params.get("task_id").unwrap();
    let queue = params.get("queue").unwrap();
    let url = format!("/queues/{queue}/tasks/{task_id}");
    let task = LocalResource::new(move || {
        let url = url.clone();
        async move {
            let resp: RawTask = ApiClient::get(&url).await.ok()?;
            Some(resp)
        }
    });
    let logs = RwSignal::new(vec![]);

    let sse = use_sse_provider();

    spawn_local(async move {
        let ev = sse.event_source();
        let mut stream = ev
            .to_stream()
            .filter(move |log: &LogEntry| {
                let matches = log
                    .span
                    .as_ref()
                    .map(|s| s.task_id == task_id)
                    .unwrap_or(false);
                async move { matches }
            })
            .boxed_local();
        while let Some(next) = stream.next().await {
            if next
                .entry
                .message
                .as_ref()
                .is_some_and(|m| m.starts_with("task."))
            {
                // Refresh task details on task.* events
                task.refetch();
            }
            logs.update(|list| {
                list.push(next);
            });
        }
    });

    view! {
        <div class="flex flex-col w-full">
            <div class="w-full">
                {move || {
                    task.get()
                        .map(|task_opt| {
                            if let Some(task) = task_opt {
                                view! { <SingleTaskView task logs=logs queue=queue.clone() /> }
                                    .into_any()
                            } else {
                                view! {
                                    <div class="text-center text-charcoal-400 dark:text-charcoal-300">
                                        "Task not found."
                                    </div>
                                }
                                    .into_any()
                            }
                        })
                        .unwrap_or_else(|| {
                            view! {
                                <div class="text-center text-charcoal-400 dark:text-charcoal-300">
                                    "Loading..."
                                </div>
                            }
                                .into_any()
                        })
                }}
            </div>
        </div>
    }
}

#[component]
pub fn LogViewer(items: RwSignal<Vec<LogEntry>>) -> impl IntoView {
    let (search_term, set_search_term) = signal(String::new());
    let (current_filter, set_current_filter) = signal(None::<LogLevel>);

    let filtered_logs = Memo::new(move |_| {
        let search = search_term.get().to_lowercase();
        let filter = current_filter.get();

        items
            .get()
            .into_iter()
            .filter(|log| {
                let matches_filter = filter.is_none() || Some(&log.level) == filter.as_ref();
                let matches_search = search.is_empty()
                    || log
                        .entry
                        .message
                        .as_ref()
                        .is_some_and(|m| m.to_lowercase().contains(&search))
                    || log.timestamp.contains(&search);
                matches_filter && matches_search
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class="p-4 font-mono bg-charcoal-900 text-gray-200 overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-700 hover:scrollbar-thumb-charcoal-600">
            <div class="max-w-6xl mx-auto">
                <header class="mb-3 flex items-center justify-between gap-4 bg-charcoal-900 py-4 z-10 w-full">
                    <h1 class="text-base text-gray-300">"task.log"</h1>
                    <div class="flex items-center gap-3 text-sm">
                        <input
                            type="text"
                            placeholder="search..."
                            class="bg-charcoal-900 border border-charcoal-700 px-2 py-1 rounded-none text-gray-200 placeholder-charcoal-500 focus:outline-none focus:border-charcoal-500 w-48"
                            on:input=move |ev| {
                                set_search_term.set(event_target_value(&ev));
                            }
                            prop:value=search_term
                        />
                        <div class="flex gap-2">
                            <button
                                class=move || {
                                    if current_filter.get().is_none() {
                                        "px-2 py-1 rounded-none text-charcoal-400 hover:text-gray-200 font-bold border border-charcoal-700 bg-charcoal-900"
                                    } else {
                                        "px-2 py-1 rounded-none text-charcoal-400 hover:text-gray-200 border border-charcoal-700 bg-charcoal-900"
                                    }
                                }
                                on:click=move |_| set_current_filter.set(None)
                            >
                                "all"
                            </button>
                            <button
                                class=move || {
                                    let base = "px-2 py-1 rounded-none border border-charcoal-700 bg-charcoal-900 "
                                        .to_string();
                                    let color = log_level_button_color(&LogLevel::Info);
                                    if current_filter.get() == Some(LogLevel::Info) {
                                        format!("{base}{color} font-bold")
                                    } else {
                                        format!("{base}{color}")
                                    }
                                }
                                on:click=move |_| set_current_filter.set(Some(LogLevel::Info))
                            >
                                "info"
                            </button>
                            <button
                                class=move || {
                                    let base = "px-2 py-1 rounded-none border border-charcoal-700 bg-charcoal-900 "
                                        .to_string();
                                    let color = log_level_button_color(&LogLevel::Success);
                                    if current_filter.get() == Some(LogLevel::Success) {
                                        format!("{base}{color} font-bold")
                                    } else {
                                        format!("{base}{color}")
                                    }
                                }
                                on:click=move |_| set_current_filter.set(Some(LogLevel::Success))
                            >
                                "success"
                            </button>
                            <button
                                class=move || {
                                    let base = "px-2 py-1 rounded-none border border-charcoal-700 bg-charcoal-900 "
                                        .to_string();
                                    let color = log_level_button_color(&LogLevel::Warn);
                                    if current_filter.get() == Some(LogLevel::Warn) {
                                        format!("{base}{color} font-bold")
                                    } else {
                                        format!("{base}{color}")
                                    }
                                }
                                on:click=move |_| set_current_filter.set(Some(LogLevel::Warn))
                            >
                                "warn"
                            </button>
                            <button
                                class=move || {
                                    let base = "px-2 py-1 rounded-none border border-charcoal-700 bg-charcoal-900 "
                                        .to_string();
                                    let color = log_level_button_color(&LogLevel::Error);
                                    if current_filter.get() == Some(LogLevel::Error) {
                                        format!("{base}{color} font-bold")
                                    } else {
                                        format!("{base}{color}")
                                    }
                                }
                                on:click=move |_| set_current_filter.set(Some(LogLevel::Error))
                            >
                                "error"
                            </button>
                        </div>
                    </div>
                </header>

                <div class="font-mono text-sm leading-relaxed">
                    <For
                        each=move || filtered_logs.get()
                        key=|log| {
                            format!(
                                "{}-{}",
                                log.timestamp,
                                log.entry.message.clone().unwrap_or_default(),
                            )
                        }
                        children=move |log: LogEntry| {
                            let attempt = log.span.map(|s| s.attempt).unwrap_or(0);
                            view! {
                                <div class="hover:bg-charcoal-900 px-1 -mx-1 rounded">
                                    <span class="text-charcoal-500">{log.timestamp}</span>
                                    <span class=format!(
                                        "{} mx-2",
                                        log_level_color(&log.level),
                                    )>{log_level_symbol(&log.level)}</span>
                                    <span class="text-charcoal-400">
                                        "[attempt: " {attempt} "] "
                                    </span>
                                    <span class="text-gray-200">{log.entry.message}</span>
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

fn log_level_color(level: &LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "text-blue-400",
        LogLevel::Success => "text-green-400",
        LogLevel::Warn => "text-yellow-400",
        LogLevel::Error => "text-red-400",
        LogLevel::Debug => "text-gray-400",
        LogLevel::Trace => "text-purple-400",
    }
}

fn log_level_symbol(level: &LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "ℹ",
        LogLevel::Success => "✓",
        LogLevel::Warn => "⚠",
        LogLevel::Error => "✗",
        LogLevel::Debug => "●",
        LogLevel::Trace => "◆",
    }
}

fn log_level_as_str(level: &LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "info",
        LogLevel::Success => "success",
        LogLevel::Warn => "warning",
        LogLevel::Error => "error",
        LogLevel::Debug => "debug",
        LogLevel::Trace => "trace",
    }
}

fn log_level_button_color(level: &LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "text-blue-500 hover:text-blue-400",
        LogLevel::Success => "text-green-500 hover:text-green-400",
        LogLevel::Warn => "text-yellow-500 hover:text-yellow-400",
        LogLevel::Error => "text-red-500 hover:text-red-400",
        LogLevel::Debug => "text-gray-500 hover:text-gray-400",
        LogLevel::Trace => "text-purple-500 hover:text-purple-400",
    }
}
