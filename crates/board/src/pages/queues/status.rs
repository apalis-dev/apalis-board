use std::str::FromStr;

use apalis_core::task::status::Status;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_struct_table::{DisplayStrategy, PaginationController, SortingMode, TableContent};

use crate::{
    components::{pagination::Paginator, sidebar::queues_icon},
    pages::{queues::index::QueueNav, tasks::provider::TaskProvider},
};

#[component]
pub fn StatusPage() -> impl IntoView {
    let params = use_params_map();
    let queue = move || params.with(|p| p.get("queue").unwrap().clone());
    let queue_name = move || queue();
    let status = move || {
        params.with(|p| {
            p.get("status")
                .map(|s| Status::from_str(&s).unwrap())
                .unwrap_or_default()
        })
    };
    let rows = move || TaskProvider::new_with_status(queue(), status());
    let pagination_controller = PaginationController::default();
    view! {
        <div class="flex flex-col h-full w-full overflow-y-auto">
            <div class="w-full bg-background-bright border-b border-gray-700 flex items-center h-[2.75rem]">
                <span class="ml-2 mr-1 rounded p-1 bg-charcoal-700 text-text-bright p-2">
                    {queues_icon()}
                </span>
                <h3 class="text-lg font-bold m-2">{queue_name}</h3>
            </div>
            <div class="flex items-center gap-1 p-2 border-b border-gray-700">
                <QueueNav />
            </div>
            <div class="flex-1 overflow-hidden overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-600 hover:scrollbar-thumb-charcoal-500 w-full">
                <table class="whitespace-nowrap transition-all duration-200 w-full">
                    {move || {
                        view! {
                            <TableContent
                                rows=rows()
                                scroll_container="html"
                                sorting_mode=SortingMode::SingleColumn
                                display_strategy=DisplayStrategy::Pagination {
                                    controller: pagination_controller,
                                    row_count: 20,
                                }
                            />
                        }
                    }}
                </table>
            </div>
            <Paginator pagination_controller />
        </div>
    }
}
