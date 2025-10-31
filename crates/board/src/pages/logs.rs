use apalis_board_types::LogEntry;
use futures::{StreamExt, future::ready};
use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::Title;

use crate::{pages::tasks::single::LogViewer, use_sse_provider};

#[component]
pub fn LogsPage() -> impl IntoView {
    let logs = RwSignal::new(Vec::new());

    let sse = use_sse_provider();

    spawn_local(async move {
        let ev = sse.event_source();
        let mut stream = ev
            .to_stream()
            .filter(move |log: &LogEntry| ready(log.span.is_some()))
            .boxed_local();
        while let Some(next) = stream.next().await {
            logs.update(|list| {
                list.push(next);
            });
        }
    });
    view! {
        <Title text="Logs" />
        <div class="h-full w-full overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-700 hover:scrollbar-thumb-charcoal-600">
            <LogViewer items=logs title="Logs" show_id=true />
        </div>
    }
}
