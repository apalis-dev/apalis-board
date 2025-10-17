use apalis_board::components::layout::Layout;
use apalis_board::components::not_found::NotFound;
use apalis_board::create_sse_resource;
use apalis_board::pages::home::Home;
use apalis_board::pages::logs::LogsPage;
use apalis_board::pages::queues::index::QueuePage;
use apalis_board::pages::queues::single::SingleQueuePage;
use apalis_board::pages::queues::status::StatusPage;
use apalis_board::pages::tasks::index::AllTasksPage;
use apalis_board::pages::tasks::single::TaskPage;
use apalis_board::pages::workers::index::WorkersPage;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use apalis_board::i18n::I18nContextProvider;

#[component]
pub fn AppRoutes() -> impl IntoView {
    leptos_meta::provide_meta_context();
    let sse_provider = create_sse_resource("/api/v1/events");
    provide_context(sse_provider);

    view! {
        <I18nContextProvider>
            <Router>
                <Routes fallback=NotFound>
                    <ParentRoute path=path!("") view=Layout>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/queues") view=QueuePage />
                        <Route path=path!("/queues/:queue") view=SingleQueuePage />
                        <Route path=path!("/queues/:queue/:status") view=StatusPage />
                        <Route path=path!("/queues/:queue/tasks/:task_id") view=TaskPage />
                        <Route path=path!("/tasks") view=AllTasksPage />
                        <Route path=path!("/tasks/:status") view=AllTasksPage />
                        <Route path=path!("/workers") view=WorkersPage />
                        <Route path=path!("/logs") view=LogsPage />
                    </ParentRoute>
                </Routes>
            </Router>
        </I18nContextProvider>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <AppRoutes /> })
}
