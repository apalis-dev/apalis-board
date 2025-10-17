use leptos::prelude::*;
use leptos_struct_table::*;

use crate::components::sidebar::workers_icon;
use crate::pages::workers::provider::WorkerProvider;

#[component]
pub fn WorkersPage() -> impl IntoView {
    let rows = WorkerProvider::all();

    view! {
        <div class="flex flex-col w-full">
            <div class="w-full bg-background-bright border-b border-gray-700 flex items-center">
                <span class="ml-2 mr-1 rounded p-1 bg-charcoal-700 text-text-bright p-2">
                    {workers_icon()}
                </span>
                <h3 class="text-base font-bold m-2">Workers</h3>
            </div>
            <div class="flex-1 overflow-hidden overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-charcoal-600 hover:scrollbar-thumb-charcoal-500 w-full">
                <table class="whitespace-nowrap transition-all duration-200 w-full">

                    <TableContent
                        rows=rows
                        scroll_container="html"
                        sorting_mode=SortingMode::SingleColumn
                    />

                </table>
            </div>
        </div>
    }
}
