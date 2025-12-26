use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use leptos_struct_table::*;

use crate::components::sidebar::queues_icon;
use crate::pages::queues::index::QueueNav;
use crate::pages::workers::provider::WorkerProvider;

#[component]
pub fn SingleQueuePage() -> impl IntoView {
    let params = use_params_map();
    let queue = Signal::derive(move || params.with(|p| p.get("queue")).unwrap_or_default());

    // Make rows reactive by using a closure
    let rows = WorkerProvider::new(queue);
    // let pagination_controller = PaginationController::default();
    view! {
        <Title text=move || format!("Queue - {}", queue.get()) />
        <div class="flex flex-col h-full w-full">
            <div class="w-full bg-background-bright border-b border-gray-700 flex items-center h-[2.75rem]">
                <span class="ml-2 mr-1 rounded p-1 bg-charcoal-700 text-text-bright p-2">
                    {queues_icon()}
                </span>
                <h3 class="text-base font-bold m-2">{queue}</h3>
            </div>
            <div class="flex items-center gap-1 p-2 border-b border-gray-700">
                <QueueNav />
            </div>
            <div class="flex-1 overflow-hidden overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-600 hover:scrollbar-thumb-charcoal-500 w-full">
                <table class="whitespace-nowrap transition-all duration-200 w-full">

                    <TableContent
                        rows=rows
                        scroll_container="html"
                        sorting_mode=SortingMode::SingleColumn
                    />
                // display_strategy=DisplayStrategy::Pagination {
                // controller: pagination_controller,
                // row_count: 10,
                // }

                // row_renderer=CustomTableRowRenderer
                </table>
            </div>
        // <Paginator pagination_controller />
        </div>
    }
}
