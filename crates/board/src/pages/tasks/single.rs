use crate::{
    RawTask,
    api::ApiClient,
    components::sidebar::{logs_icon, tasks_icon},
    pages::tasks::MetaKey,
    use_sse_provider,
};
use apalis_board_types::{LogEntry, LogLevel};
use apalis_core::task::status::Status;
use futures::StreamExt;
use leptos::{leptos_dom::logging::console_debug_log, prelude::*, reactive::spawn_local};
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
                console_debug_log(&format!("{:?}", obj));
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
            <div class="w-full bg-background-bright border-b border-charcoal-700 flex items-center h-[2.75rem]">
                <span class="m-2 rounded-sm p-1.5 bg-charcoal-700 text-text-bright">
                    {tasks_icon()}
                </span>
                <h3 class="text-base font-bold m-2">{queue}</h3>
                <span class="text-gray-500">"/"</span>
                <h4 class="text-sm m-2 flex-1">{task_id.clone()}</h4>
                <div class="flex items-center gap-3 mr-4">
                    <span class=format!(
                        "inline-flex items-center px-3 py-1 rounded-sm text-xs font-medium border border-charcoal-700 {}",
                        status_class,
                    )>{task.parts.status.load().to_string()}</span>
                    <span class="py-1 px-2 inline-flex items-center gap-x-1 text-xs font-medium rounded-sm border border-charcoal-700">
                        "Attempt: " {task.parts.attempt.current()}
                    </span>
                </div>
            </div>

            <div class="lg:grid lg:grid-cols-5 flex-1 overflow-y-auto">

                <div class="lg:col-span-2 flex flex-col overflow-y-auto border-r border-charcoal-700">
                    <div class="bg-charcoal-900">
                        <h2 class="p-2 text-base h-[2.75rem]">"Arguments"</h2>
                        <pre class="p-3 overflow-auto dark:border-charcoal-650 border-t scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-600 hover:scrollbar-thumb-charcoal-500">
                            <code class="text-xs text-gray-300 font-mono">{args_json}</code>
                        </pre>
                    </div>

                    <div class="bg-charcoal-900 border-t border-charcoal-700 overflow-y-auto flex flex-col flex-1">
                        <h2 class="p-2 text-base flex">"Context"</h2>

                        <div class="flex-1 space-y-2 border-t border-charcoal-700 dark:border-charcoal-650 p-3 overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-600 hover:scrollbar-thumb-charcoal-500">
                            {items()
                                .into_iter()
                                .map(move |val| {
                                    let has_title = !val.render_key().is_empty();
                                    view! {
                                        <div class="flex justify-between items-start py-2 border-b border-charcoal-700 last:border-b-0">
                                            {if has_title {
                                                view! {
                                                    <span class="text-sm text-charcoal-400 dark:text-charcoal-300">
                                                        {val.render_key().to_string()}
                                                    </span>
                                                }
                                                    .into_any()
                                            } else {
                                                view! { <></> }.into_any()
                                            }}
                                            <span class=format!(
                                                "text-charcoal-100 dark:text-charcoal-100 truncate {}",
                                                (!has_title).then(|| "w-full").unwrap_or_default(),
                                            )>{val.render_full()}</span>
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>
                </div>
                <div class="lg:col-span-3 h-full overflow-auto">
                    <LogViewer title="Events" items=logs />
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
        <>
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
        </>
    }
}

#[component]
pub fn LogViewer(
    items: RwSignal<Vec<LogEntry>>,
    title: &'static str,
    #[prop(optional)] show_id: bool,
) -> impl IntoView {
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
                    || log.timestamp.contains(&search)
                    || log
                        .span
                        .as_ref()
                        .map(|s| s.task_id.to_lowercase().contains(&search))
                        .unwrap_or(false);
                matches_filter && matches_search
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class="flex flex-col h-full bg-charcoal-900 text-gray-200 overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-700 hover:scrollbar-thumb-charcoal-600">
            <header class="p-2 flex w-full h-[2.75rem]">
                <span class="mr-2 rounded p-1 bg-charcoal-700 text-text-bright">{logs_icon()}</span>
                <h2 class="text-base">{title}</h2>
            </header>
            <div class="flex p-2 w-full border-t border-charcoal-700">
                <div class="flex items-center gap-3 text-sm">
                    <input
                        type="text"
                        placeholder="search..."
                        class="flex-1 bg-charcoal-900 border border-charcoal-700 px-2 py-1 rounded-sm text-gray-200 placeholder-charcoal-500 focus:outline-none focus:border-charcoal-500 w-48"
                        on:input=move |ev| {
                            set_search_term.set(event_target_value(&ev));
                        }
                        prop:value=search_term
                    />
                    <div class="flex gap-2">
                        <button
                            class=move || {
                                if current_filter.get().is_none() {
                                    "px-2 py-1 rounded-sm text-charcoal-400 hover:text-gray-200 font-bold border border-charcoal-700 bg-charcoal-900"
                                } else {
                                    "px-2 py-1 rounded-sm text-charcoal-400 hover:text-gray-200 border border-charcoal-700 bg-charcoal-900"
                                }
                            }
                            on:click=move |_| set_current_filter.set(None)
                        >
                            "all"
                        </button>
                        <button
                            class=move || {
                                let base = "px-2 py-1 rounded-sm border border-charcoal-700 bg-charcoal-900 "
                                    .to_string();
                                let color = log_level_button_color(&LogLevel::Debug);
                                if current_filter.get() == Some(LogLevel::Debug) {
                                    format!("{base}{color} font-bold")
                                } else {
                                    format!("{base}{color}")
                                }
                            }
                            on:click=move |_| set_current_filter.set(Some(LogLevel::Debug))
                        >
                            "debug"
                        </button>
                        <button
                            class=move || {
                                let base = "px-2 py-1 rounded-sm border border-charcoal-700 bg-charcoal-900 "
                                    .to_string();
                                let color = log_level_button_color(&LogLevel::Trace);
                                if current_filter.get() == Some(LogLevel::Trace) {
                                    format!("{base}{color} font-bold")
                                } else {
                                    format!("{base}{color}")
                                }
                            }
                            on:click=move |_| set_current_filter.set(Some(LogLevel::Trace))
                        >
                            "trace"
                        </button>
                        <button
                            class=move || {
                                let base = "px-2 py-1 rounded-sm border border-charcoal-700 bg-charcoal-900 "
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
                                let base = "px-2 py-1 rounded-sm border border-charcoal-700 bg-charcoal-900 "
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
                                let base = "px-2 py-1 rounded-sm border border-charcoal-700 bg-charcoal-900 "
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

            </div>
            <div class="font-mono p-2 text-sm flex-1 leading-relaxed overflow-auto dark:border-charcoal-650 border-t scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-600 hover:scrollbar-thumb-charcoal-500">
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
                        let attempt = log.span.as_ref().map(|s| s.attempt).unwrap_or(0);
                        view! {
                            <div class="hover:bg-charcoal-900 px-1 -mx-1 rounded">
                                {if show_id {
                                    view! {
                                        <span class="text-charcoal-500 mr-2">
                                            {format!(
                                                "[{}] ",
                                                log
                                                    .span
                                                    .as_ref()
                                                    .map(|s| s.task_id.clone())
                                                    .unwrap_or_default(),
                                            )}
                                        </span>
                                    }
                                        .into_any()
                                } else {
                                    view! {}.into_any()
                                }} <span class="text-charcoal-500">{log.timestamp}</span>
                                <span class=format!(
                                    "{} mx-2",
                                    log_level_color(&log.level),
                                )>{log_level_symbol(&log.level)}</span>
                                <span
                                    title="Attempts"
                                    class="bg-charcoal-800 text-charcoal-400 text-xs font-medium me-2 px-2.5 py-0.5 rounded-sm"
                                >
                                    {attempt}
                                </span> <span class="text-gray-200">{log.entry.message}</span>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

fn log_level_color(level: &LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "text-blue-400",
        LogLevel::Warn => "text-yellow-400",
        LogLevel::Error => "text-red-400",
        LogLevel::Debug => "text-gray-400",
        LogLevel::Trace => "text-purple-400",
    }
}

fn log_level_symbol(level: &LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "ℹ",
        LogLevel::Warn => "⚠",
        LogLevel::Error => "✗",
        LogLevel::Debug => "●",
        LogLevel::Trace => "◆",
    }
}

fn log_level_button_color(level: &LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "text-blue-500 hover:text-blue-400",
        LogLevel::Warn => "text-yellow-500 hover:text-yellow-400",
        LogLevel::Error => "text-red-500 hover:text-red-400",
        LogLevel::Debug => "text-gray-500 hover:text-gray-400",
        LogLevel::Trace => "text-purple-500 hover:text-purple-400",
    }
}
