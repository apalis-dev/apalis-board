use apalis_board_web::components::layout::Layout;
use apalis_board_web::components::not_found::NotFound;
use apalis_board_web::create_sse_resource;
use apalis_board_web::pages::home::Home;
use apalis_board_web::pages::logs::LogsPage;
use apalis_board_web::pages::queues::index::QueuePage;
use apalis_board_web::pages::queues::single::SingleQueuePage;
use apalis_board_web::pages::queues::status::StatusPage;
use apalis_board_web::pages::tasks::index::AllTasksPage;
use apalis_board_web::pages::tasks::single::TaskPage;
use apalis_board_web::pages::workers::index::WorkersPage;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use apalis_board_web::locales::i18n::I18nContextProvider;

/// Main application routes component
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
