use std::{marker::PhantomData, str::FromStr};

use actix_web::{
    HttpResponse, Responder, Scope,
    web::{self, Data, Json},
};
use apalis_core::backend::{
    Backend, BackendExt, FetchById, Filter, ListAllTasks, ListQueues, ListTasks,
    ListWorkers, Metrics, TaskSink, codec::Codec,
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::RwLock;

use crate::{
    fetch_queues,
    framework::{ApiBuilder, RegisterRoute},
    get_all_tasks, get_all_workers, get_task_by_id, get_tasks, get_workers, overview, push_task,
    stats_by_queue,
};

#[cfg(feature = "ui")]
use crate::ui::ServeUI;

/// Handler struct for Actix web routes.
#[derive(Debug, Clone)]
pub struct Handler<S, T, Compact> {
    _phantom: PhantomData<(S, T, Compact)>,
}

impl<S, T, Compact> Handler<S, T, Compact> {
    /// Get tasks for a specific queue.
    pub async fn get_tasks(
        queue: web::Data<String>,
        storage: web::Data<RwLock<S>>,
        query: web::Query<Filter>,
    ) -> impl Responder
    where
        T: Serialize + DeserializeOwned + 'static,
        S: ListTasks<T> + Send + 'static + BackendExt,
        S::Context: Serialize + 'static,
        S::IdType: Serialize + 'static,
        <S as Backend>::Error: std::error::Error + 'static,
        S::Codec: Codec<T, Compact = Compact> + 'static,
        Compact: 'static,
    {
        let queue = queue.into_inner().to_string();
        let storage = storage.into_inner();
        let filter = query.into_inner();

        match get_tasks::<S, T, Compact>(queue, storage, filter).await {
            Ok(tasks) => HttpResponse::Ok().json(tasks),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

    /// Get statistics for a specific queue.
    pub async fn stats_by_queue(
        queue: web::Data<String>,
        storage: web::Data<RwLock<S>>,
    ) -> impl Responder
    where
        S::Error: std::error::Error,
        S: Metrics,
    {
        let queue = queue.into_inner();
        let storage = storage.into_inner();

        match stats_by_queue::<S>(storage, queue.to_string()).await {
            Ok(stats) => HttpResponse::Ok().json(stats),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

    /// Get workers for a specific queue.
    pub async fn get_workers(
        queue: web::Data<String>,
        storage: web::Data<RwLock<S>>,
    ) -> impl Responder
    where
        S: ListWorkers,
        S::Error: std::error::Error,
    {
        let queue = queue.into_inner().to_string();
        let storage = storage.into_inner();

        match get_workers::<S>(storage, queue).await {
            Ok(workers) => HttpResponse::Ok().json(workers),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

    /// Push a new task to the specified queue.
    pub async fn push_task(
        queue: web::Data<String>,
        task: Json<T>,
        storage: Data<RwLock<S>>,
    ) -> impl Responder
    where
        T: Serialize + DeserializeOwned + 'static,
        S: TaskSink<T> + Send + BackendExt,
        S::Error: std::error::Error,
        S::Codec: Codec<T, Compact = Compact>,
        <<S as BackendExt>::Codec as Codec<T>>::Error: std::error::Error,
    {
        let queue = queue.into_inner().to_string();
        match push_task(queue, task.into_inner(), storage.into_inner()).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

    /// Get a task by its ID.
    pub async fn get_task_by_id(
        task_id: web::Path<String>,
        storage: web::Data<RwLock<S>>,
    ) -> impl Responder
    where
        T: Serialize + DeserializeOwned + 'static,
        S: FetchById<T> + 'static,
        S::Context: Serialize,
        S::IdType: Serialize,
        S::Context: Serialize,
        S::Error: std::error::Error,
        S::IdType: FromStr,
        <<S as Backend>::IdType as FromStr>::Err: std::error::Error,
    {
        let task_id = task_id.into_inner();
        let storage = storage.into_inner();

        match get_task_by_id::<S, T>(task_id, storage).await {
            Ok(Some(task)) => HttpResponse::Ok().json(task),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

    /// Get all tasks across all queues.
    pub async fn get_all_tasks(
        storage: web::Data<RwLock<S>>,
        query: web::Query<Filter>,
    ) -> impl Responder
    where
        S: ListAllTasks + Send,
        S::Context: Serialize,
        S::IdType: Serialize,
        S::Compact: Serialize,
        <S as Backend>::Error: std::error::Error,
        <<S as BackendExt>::Codec as Codec<<S as Backend>::Args>>::Error: std::error::Error,
    {
        let storage = storage.into_inner();
        let filter = query.into_inner();

        match get_all_tasks::<S>(storage, filter).await {
            Ok(tasks) => HttpResponse::Ok().json(tasks),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

    /// Get all workers across all queues.
    pub async fn get_all_workers(storage: web::Data<RwLock<S>>) -> impl Responder
    where
        S: ListWorkers,
        S::Error: std::error::Error,
    {
        let storage = storage.into_inner();

        match get_all_workers::<S>(storage).await {
            Ok(workers) => HttpResponse::Ok().json(workers),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

    /// Fetch all queues.
    pub async fn fetch_queues(storage: web::Data<RwLock<S>>) -> impl Responder
    where
        S::Error: std::error::Error,
        S: ListQueues,
    {
        let storage = storage.into_inner();

        match fetch_queues::<S>(storage).await {
            Ok(queues) => HttpResponse::Ok().json(queues),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

    /// Get an overview of statistics.
    pub async fn overview(storage: web::Data<RwLock<S>>) -> impl Responder
    where
        S::Error: std::error::Error,
        S: Metrics,
    {
        let storage = storage.into_inner();

        match overview::<S>(storage).await {
            Ok(stats) => HttpResponse::Ok().json(stats),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }
}

impl<B, T, Compact> RegisterRoute<B, T> for ApiBuilder<Scope>
where
    B: Metrics + ListWorkers + ListAllTasks + ListQueues + Send + 'static,
    B::Context: Serialize,
    B::IdType: Serialize,
    <B as Backend>::Error: std::error::Error,
    B::IdType: FromStr,
    <<B as Backend>::IdType as FromStr>::Err: std::error::Error,
    Compact: Serialize + 'static,
    B::Compact: Serialize,
    <B as Backend>::Error: std::error::Error,
    <<B as BackendExt>::Codec as Codec<<B as Backend>::Args>>::Error: std::error::Error,
    T: Serialize + DeserializeOwned + 'static,
    B: ListTasks<T> + FetchById<T>,
    B::Codec: Codec<T, Compact = Compact>,
    <<B as BackendExt>::Codec as Codec<T>>::Error: std::error::Error,
    B: TaskSink<T>,
{
    fn register(mut self, backend: B) -> Self {
        let queue = backend.get_queue();
        let backend = web::Data::new(RwLock::new(backend));
        if self.root {
            #[allow(unused_mut)]
            let mut router = self
                .router
                .app_data(backend.clone())
                .route("/", web::get().to(Handler::<B, (), Compact>::fetch_queues))
                .route(
                    "/tasks",
                    web::get().to(Handler::<B, (), Compact>::get_all_tasks),
                )
                .route(
                    "/workers",
                    web::get().to(Handler::<B, (), Compact>::get_all_workers),
                )
                .route(
                    "/overview",
                    web::get().to(Handler::<B, (), Compact>::overview),
                );

            #[cfg(feature = "sse")]
            {
                router = router.route("/events", web::get().to(sse::new_client));
            }

            self.router = router;
        }
        let scope = self.router.service(
            Scope::new(&format!("/queues/{queue}"))
                .app_data(web::Data::new(queue.to_string()))
                .app_data(backend)
                .route("/tasks", web::get().to(Handler::<B, T, Compact>::get_tasks))
                .route(
                    "/stats",
                    web::get().to(Handler::<B, T, Compact>::stats_by_queue),
                )
                .route(
                    "/workers",
                    web::get().to(Handler::<B, T, Compact>::get_workers),
                )
                .route("/tasks", web::put().to(Handler::<B, T, Compact>::push_task)) // Allow add jobs via api
                .route(
                    "/tasks/{id}",
                    web::get().to(Handler::<B, T, Compact>::get_task_by_id),
                ),
        );

        Self {
            router: scope,
            root: false,
        }
    }
}

#[cfg(feature = "ui")]
mod ui {
    use super::ServeUI;
    use actix_web::{
        HttpRequest, HttpResponse, HttpResponseBuilder,
        dev::HttpServiceFactory,
        http::{StatusCode, header},
    };
    impl ServeUI {
        fn serve_file(path: &str) -> HttpResponse {
            let mut file = Self::get_file(path);
            if file.is_none() {
                // Try fallback to index.html for unknown routes
                file = Self::get_file("index.html");
            }

            match file {
                Some(f) => {
                    let path_str = f.path().to_str().unwrap_or("");
                    let mut builder = HttpResponse::Ok();
                    let mut builder =
                        builder.insert_header((header::CONTENT_TYPE, Self::content_type(path_str)));

                    if let Some(cache) = Self::cache_control(path_str) {
                        builder = builder.insert_header((header::CACHE_CONTROL, cache));
                    }

                    builder.body(f.contents().to_vec())
                }
                None => HttpResponseBuilder::new(StatusCode::NOT_FOUND).finish(),
            }
        }
    }
    impl HttpServiceFactory for ServeUI {
        fn register(self, config: &mut actix_web::dev::AppService) {
            let resource = actix_web::Resource::new("/{tail:.*}").route(actix_web::web::get().to(
                move |req: HttpRequest| async move {
                    let path = req.match_info().query("tail");

                    Self::serve_file(path)
                },
            ));
            resource.register(config);
        }
    }
}

/// Expose Server-Sent Events (SSE) functionality.
#[cfg(feature = "sse")]
pub mod sse {
    use std::{sync::Arc, time::Duration};

    use crate::sse::TracingBroadcaster;
    use actix_web::web::*;
    use actix_web_lab::sse::Event;
    use futures::StreamExt;
    use std::sync::Mutex;

    /// Create a new SSE client connection.
    pub async fn new_client(
        broadcaster: Data<Arc<Mutex<TracingBroadcaster>>>,
    ) -> impl actix_web::Responder {
        let rx = broadcaster.lock().unwrap().new_client();

        actix_web_lab::sse::Sse::from_stream(
            rx.filter(|s| futures::future::ready(s.as_ref().is_ok_and(|e| e.span.is_some())))
                .map(|entry| {
                    match actix_web_lab::sse::Data::new_json(
                        entry.map_err(actix_web::error::ErrorInternalServerError)?,
                    ) {
                        Ok(data) => Ok(Event::Data(data)),
                        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
                    }
                }),
        )
        .with_keep_alive(Duration::from_secs(60 * 5))
    }
}
