use std::str::FromStr;

use apalis_core::task::status::Status;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};
use leptos_struct_table::*;

use crate::{
    components::{pagination::Paginator, sidebar::queues_icon},
    pages::queues::{provider::QueueProvider, CustomTableRowRenderer},
};

#[component]
pub fn SearchBox() -> impl IntoView {
    let (search, set_search) = signal(String::new());

    let on_input = move |ev: leptos::ev::Event| {
        let input = event_target_value(&ev);
        {
            set_search.set(input.clone());
            // on_search.call(input); TODO
        }
    };

    view! {
        <div class="flex items-center has-[:focus-visible]:outline-none has-[:focus-visible]:ring-1 has-[:focus-visible]:ring-charcoal-650 has-[:focus-visible]:ring-offset-0 has-[:focus]:border-ring has-[:focus]:outline-none has-[:focus]:ring-1 has-[:focus]:ring-ring has-[:disabled]:cursor-not-allowed has-[:disabled]:opacity-50 ring-offset-background transition cursor-text px-1 h-6 rounded hover:bg-charcoal-750 w-full">
            <div class="pointer-events-none flex items-center">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                    class="size-3 ml-0.5 text-text-dimmed"
                >
                    <path
                        fill-rule="evenodd"
                        d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            </div>
            <input
                class="grow h-full w-full text-text-bright bg-transparent file:border-0 file:bg-transparent file:text-base file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-0 disabled:cursor-not-allowed outline-none ring-0 border-none [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:m-0 [&::-webkit-inner-spin-button]:m-0 [&]:[-moz-appearance:textfield] px-1 rounded text-xs"
                placeholder="Search Queues..."
                autofocus
                prop:value=search
                on:input=on_input
            />
        </div>
    }
}

#[component]
pub fn QueuePage() -> impl IntoView {
    let rows = QueueProvider::new();
    let pagination_controller = PaginationController::default();
    view! {
        <div class="flex flex-col w-full">
            <div class="w-full bg-background-bright border-b border-gray-700 flex items-center">
                <span class="ml-2 mr-1 rounded p-1 bg-charcoal-700 text-text-bright p-2">
                    {queues_icon()}
                </span>
                <h3 class="text-base font-bold m-2 flex-1">Queues</h3>
                <div class="justify-center flex items-center gap-x-2.5 mr-4 rounded-md border border-transparent bg-primary px-2.5 py-1 text-sm font-medium text-background-bright shadow-sm hover:bg-primary/80 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 transition-colors cursor-pointer">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-folder-plus-icon lucide-folder-plus"><path d="M12 10v6"/><path d="M9 13h6"/><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>
                    <span class="text-xs">"Add Task"</span>
                </div>
            </div>
            <div class="flex items-center gap-1 p-2 border-b border-gray-700">
                <SearchBox />
            </div>
            <div class="flex-1 overflow-hidden overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-600 hover:scrollbar-thumb-charcoal-500 w-full">
                <table class="whitespace-nowrap transition-all duration-200 w-full">
                    <TableContent
                        rows=rows
                        scroll_container="html"
                        sorting_mode=SortingMode::SingleColumn
                        display_strategy=DisplayStrategy::Pagination {
                            controller: pagination_controller,
                            row_count: 10,
                        }
                        row_renderer=CustomTableRowRenderer
                    />
                </table>
            </div>
            <Paginator pagination_controller />
        </div>
    }
}

#[component]
pub fn Card(title: String, status: String) -> impl IntoView {
    view! {
        <div class="rounded-sm border-grid-bright bg-background-bright">
            <div class="p-6 flex items-center space-x-4">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="h-6 w-6 text-blue-600"
                >
                    <path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"></path>
                </svg>
                <div>
                    <h5 class="text-base ">{title}</h5>
                    <div class="text-sm text-gray-500">"Last seen less than a minute ago"</div>
                </div>
                <div class="inline-flex w-fit items-center whitespace-nowrap rounded-full border px-2.5 py-0.5 text-xs  transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 border-transparent bg-primary text-primary-foreground hover:bg-primary/80 ml-auto">
                    {status}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn QueueNav() -> impl IntoView {
    let params = use_params_map();
    let queue = move || params.with(|p| p.get("queue").unwrap());

    let queue_home = move || format!("/queues/{}", queue());

    let worker_home = move |status: &Status| format!("/queues/{}/{}", queue(), status);

    let calculate_active = move |status: &Status| {
        let current_status =
            params.with(|p| p.get("status").map(|s| Status::from_str(&s).unwrap()));
        match current_status {
            Some(s) => s == *status,
            None => false,
        }
    };

    let is_index = move || params.with(|p| p.get("status").is_none());

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
                <div class="items-center px-1 text-sm font-medium text-center text-text-bright border-r pr-4 border-grid-bright">
                    <A
                        href=move || queue_home()
                        attr:class=move || calculate_class_active(is_index())
                    >
                        "Workers"
                    </A>
                </div>
                {statuses
                    .into_iter()
                    .map(move |status| {
                        let s = status.to_string();
                        let s1 = status.clone();
                        view! {
                            <A
                                href=move || worker_home(&status)
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
