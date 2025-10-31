use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use apalis_core::backend::{QueueInfo, Statistic};
use gloo_net::http::Request;
use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::Title;
use leptos_router::components::A;
use serde::Serialize;

use crate::{i18n::*, translate::KnownStatistic};

pub fn resolve_json<V: Serialize>(val: V) -> String {
    serde_json::to_string_pretty(&val).unwrap()
}

const API_PATH: &str = "/api/v1";

async fn queue_list() -> Result<Vec<QueueInfo>, String> {
    let res = Request::get(&format!("{API_PATH}/"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let queues: Vec<QueueInfo> = res.json().await.map_err(|e| e.to_string())?;
    Ok(queues)
}

async fn overview() -> Result<Vec<Statistic>, String> {
    let res = Request::get(&format!("{API_PATH}/overview"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let stats: Vec<Statistic> = res.json().await.map_err(|e| e.to_string())?;
    Ok(stats)
}

#[component]
pub fn Home() -> impl IntoView {
    let queues = LocalResource::new(queue_list);
    let stats = LocalResource::new(overview);

    let last_10_stats = RwSignal::new(HashMap::new());

    Effect::new(move |_| {
        use std::str::FromStr;
        if let Some(Ok(ref stats)) = stats.get() {
            last_10_stats.update(|map| {
                for stat in stats {
                    let entry: &mut VecDeque<f64> = map.entry(stat.title.clone()).or_default();
                    entry.push_back(f64::from_str(&stat.value).unwrap_or(0.0));
                    if entry.len() > 10 {
                        entry.pop_front();
                    }
                }
            });
        }
        || {}
    });

    spawn_local(async move {
        loop {
            gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
            queues.refetch();
            stats.refetch();
        }
    });
    view! {
        <Title text="Home" />
        <div class="p-4 space-y-8 w-full overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-700 hover:scrollbar-thumb-charcoal-600 transition-transform duration-200 ease-out transform">
            <section>
                <h2 class="text-base">"Overview"</h2>
                <p class="text-gray-500 text-sm">"Backend stats overview"</p>
                <div class="grid grid-cols-1 gap-4 mt-4 md:grid-cols-4">
                    {move || {
                        match stats.get() {
                            None => view! { <p>"Loading..."</p> }.into_any(),
                            Some(Ok(stats)) => {

                                view! {
                                    <>
                                        {stats
                                            .into_iter()
                                            .map(|stat| {
                                                let _extra_info = match stat.title.as_str() {
                                                    "Success rate" => Some(format!("{}%", stat.value)),
                                                    "Processed" => Some(format!("{} total", stat.value)),
                                                    _ => None,
                                                };
                                                let _extra_class = match stat.title.as_str() {
                                                    "Success rate" if stat.value.parse::<f64>().unwrap_or(0.0)
                                                        < 80.0 => Some("text-red-500".to_owned()),
                                                    "Success rate" if stat.value.parse::<f64>().unwrap_or(0.0)
                                                        < 95.0 => Some("text-yellow-500".to_owned()),
                                                    "Success rate" => Some("text-green-500".to_owned()),
                                                    _ => None,
                                                };
                                                stats_card(
                                                    stat.title.clone(),
                                                    stat.value.clone(),
                                                    last_10_stats
                                                        .get()
                                                        .get(&stat.title)
                                                        .cloned()
                                                        .unwrap_or_default(),
                                                )
                                            })
                                            .collect::<Vec<_>>()}
                                    </>
                                }
                                    .into_any()
                            }
                            Some(Err(err)) => view! { <div>"Error: " {err}</div> }.into_any(),
                        }
                    }}

                </div>
            </section>

            <section>
                <h2 class="text-base font-bold">"Queues"</h2>
                <p class="text-gray-500">
                    "These are all the queues you have created on this backend instance."
                </p>
                <div class="grid grid-cols-1 gap-4 mt-4 md:grid-cols-3">
                    {move || match queues.get() {
                        None => view! { <p>"Loading..."</p> }.into_any(),
                        Some(Ok(queues)) => {
                            view! {
                                <>
                                    {queues
                                        .into_iter()
                                        .map(|queue| {
                                            view! {
                                                <A href=format!(
                                                    "/queues/{}",
                                                    queue.name,
                                                )>{queue_card(queue)}</A>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </>
                            }
                                .into_any()
                        }
                        Some(Err(err)) => view! { <div>"Error: " {err}</div> }.into_any(),
                    }}
                </div>
            </section>
        </div>
    }
}

fn stats_card(title: String, value: String, last_10: VecDeque<f64>) -> impl IntoView {
    let max = last_10.iter().cloned().fold(0., f64::max);
    let i18n = use_i18n();
    let title = KnownStatistic::from_str(&title)
        .unwrap_or(KnownStatistic::TotalJobs)
        .translate(i18n);
    view! {
        <div class="rounded-sm border border-charcoal-700 text-charcoal-100 shadow-sm bg-charcoal-900">
            <div class="flex-col space-y-1.5 p-3 flex justify-between">
                <h3 class="whitespace-nowrap text-base leading-none tracking-tight">{title}</h3>
            </div>
            <div class="flex flex-row">
                <div class="p-3 flex-1">
                    <div class="text-base font-bold">{value.clone()}</div>
                </div>
                <div class="h-8 p-3">
                    <div class="grid grid-cols-10 h-6 items-end gap-0.5 rounded-sm">
                        {move || {
                            last_10
                                .iter()
                                .map(|v| {
                                    let percent = if max > 0.0 {
                                        (*v / max) * 80.0 + 20.0
                                    } else {
                                        20.0
                                    };
                                    let style = format!("height: {percent}%;");
                                    // Scale height between 20% and 100% of parent
                                    view! {
                                        <div
                                            class="w-1 bg-charcoal-600 rounded-sm"
                                            style=style
                                            title=v.to_string()
                                        ></div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn queue_card(queue: QueueInfo) -> impl IntoView {
    let info = queue
        .stats
        .iter()
        .find(|s| s.title == "PENDING_JOBS" && s.value != "0")
        .map(|s| s.value.clone())
        .unwrap_or_default()
        .to_string();
    let max = *queue.activity.iter().max().unwrap_or(&1);

    view! {
        <div
            class="rounded-sm border border-charcoal-700 text-charcoal-100 shadow-sm bg-charcoal-900"
            data-v0-t="card"
        >
            <div class="m-1 ms-0 relative p-6 flex items-center gap-x-2 text-sm font-medium">
                <h3 class="whitespace-nowrap text-base leading-none tracking-tight flex-1">
                    {queue.name}
                </h3>
                <span class="flex absolute top-0 end-0 -mt-2 -me-2" title="PENDING_JOBS">
                    <span class="animate-ping absolute inline-flex size-full rounded-full bg-red-400 opacity-75 dark:bg-red-600"></span>
                    <span class="relative inline-flex text-xs bg-red-500 text-white rounded-full py-0.5 px-1.5">
                        {info}+
                    </span>
                </span>
            </div>
            <div class="p-6 w-full">
                <div class="grid grid-cols-7 h-16 w-full items-end gap-0.5 rounded-sm">
                    {move || {
                        queue
                            .activity
                            .iter()
                            .map(|v| {
                                let percent = if max > 0 {
                                    (*v as f32 / max as f32) * 80.0 + 20.0
                                } else {
                                    20.0
                                };
                                let style = format!("height: {percent}%;");
                                // Scale height between 20% and 100% of parent
                                view! {
                                    <div
                                        class="w-full bg-gray-600 rounded"
                                        style=style
                                        title=v.to_string()
                                    ></div>
                                }
                            })
                            .collect_view()
                    }}
                </div>

                <div class="mt-2"></div>
            </div>
        </div>
    }
}
