use leptos::prelude::*;
use leptos_struct_table::*;
use serde::Deserialize;

#[component]
pub fn Paginator(pagination_controller: PaginationController) -> impl IntoView {
    let current_page = pagination_controller.current_page;
    let page_count = pagination_controller.page_count();

    let page_range = move || {
        let mut start = current_page.get().saturating_sub(2);

        let mut end = start + 5;

        if let Some(row_count) = page_count.get()
            && end > row_count {
                end = row_count;
                start = end.saturating_sub(5);
            }

        start..end
    };

    view! {
        <nav aria-label="Page navigation " class="my-4 flex justify-end px-2">
            <ul class="inline-flex -space-x-px text-sm">
                <li>
                    <a
                        href="#"
                        class="flex items-center rounded-s-sm justify-center px-3 h-8 ms-0 leading-tight text-charcoal-400 bg-charcoal-900 border border-e-0 border-charcoal-700 rounded-none hover:bg-charcoal-800 hover:text-charcoal-200 dark:bg-charcoal-900 dark:border-charcoal-700 dark:text-charcoal-200 dark:hover:bg-charcoal-800 dark:hover:text-charcoal-100"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.previous();
                        }
                    >
                        Previous
                    </a>
                </li>

                <For each=page_range key=|page| *page let:page>
                    <PageLink page pagination_controller />
                </For>

                <li>
                    <a
                        href="#"
                        class="flex items-center rounded-e-sm justify-center px-3 h-8 leading-tight text-charcoal-400 bg-charcoal-900 border border-charcoal-700 rounded-none hover:bg-charcoal-800 hover:text-charcoal-200 dark:bg-charcoal-900 dark:border-charcoal-700 dark:text-charcoal-200 dark:hover:bg-charcoal-800 dark:hover:text-charcoal-100"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.next();
                        }
                    >
                        Next
                    </a>
                </li>
            </ul>
        </nav>
    }
}

#[component]
pub fn PageLink(page: usize, pagination_controller: PaginationController) -> impl IntoView {
    let is_selected = move || pagination_controller.current_page.get() == page;

    let class = move || {
        if is_selected() {
            "flex items-center justify-center px-3 h-8 text-primary bg-background-bright border border-grid-bright rounded-none hover:bg-grid-bright hover:text-text-bright dark:bg-background-bright dark:border-grid-bright dark:text-text-bright dark:hover:bg-grid-bright dark:hover:text-text-bright"
        } else {
            "flex items-center justify-center px-3 h-8 leading-tight text-text-dimmed bg-white border border-grid-bright hover:bg-grid-bright hover:text-text-bright dark:bg-background-dimmed dark:border-grid-bright dark:text-text-dimmed dark:hover:bg-grid-bright dark:hover:text-text-bright"
        }
    };

    view! {
        <li>
            <a
                href="#"
                class=class
                on:click=move |evt| {
                    evt.prevent_default();
                    evt.stop_propagation();
                    pagination_controller.current_page.set(page);
                }
            >

                {page + 1}
            </a>
        </li>
    }
}

#[derive(Deserialize, Debug)]
pub struct MetaResponse {
    pub total: String,
    pub page: String,
    pub page_size: String,
}
