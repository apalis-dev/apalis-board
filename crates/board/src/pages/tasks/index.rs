use std::str::FromStr;

use apalis_core::task::status::Status;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};
use leptos_struct_table::{DisplayStrategy, PaginationController, SortingMode, TableContent};

use crate::{
    components::{pagination::Paginator, sidebar::tasks_icon},
    pages::tasks::provider::TaskProvider,
};

#[component]
pub fn AllTasksPage() -> impl IntoView {
    let params = use_params_map();
    let status = move || {
        params.with(|p| {
            p.get("status")
                .map(|s| Status::from_str(&s).unwrap())
                .unwrap_or_default()
        })
    };
    let rows = move || TaskProvider::all(status());
    let pagination_controller = PaginationController::default();
    view! {
        <div class="flex flex-col h-full w-full">
            <div class="w-full bg-background-bright border-b border-gray-700 flex items-center h-[2.75rem]">
                <span class="ml-2 mr-1 rounded p-1 bg-charcoal-700 text-text-bright p-2">
                    {tasks_icon()}
                </span>
                <h3 class="text-base font-bold m-2 flex-grow">Tasks</h3>
                <TaskNav />
            </div>
            <div class="flex items-center gap-1 p-2 border-b border-gray-700"></div>
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

#[component]
pub fn TaskNav() -> impl IntoView {
    let params = use_params_map();

    let tasks_page = move |status: &Status| format!("/tasks/{}", status);

    let calculate_active = move |status: &Status| {
        let current_status =
            params.with(|p| p.get("status").and_then(|s| Status::from_str(&s).ok()));
        match current_status {
            Some(s) => s == *status,
            None => Status::Pending == *status
        }
    };


    let calculate_class_active = move |active: bool| {
        if active {
            "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background focus-custom text-text-bright h-8 px-2 border-b-2 border-gray-100"
        } else {
            "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background focus-custom hover:bg-charcoal-750 hover:text-text-bright h-8 px-2 text-text-dimmed"
        }
    };

    let statuses = [
        Status::Pending,
        Status::Queued,
        Status::Running,
        Status::Done,
        Status::Failed,
        Status::Killed,
    ];

    view! {
        <div class="flex flex-col items-left transition-all ">
            <div class="flex space-x-4">
                {statuses
                    .into_iter()
                    .map(move |status| {
                        let s = status.to_string();
                        let s1 = status.clone();
                        view! {
                            <A
                                href=move || tasks_page(&status)
                                attr:class=move || calculate_class_active(calculate_active(&s1))
                            >
                                {s}
                            </A>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
