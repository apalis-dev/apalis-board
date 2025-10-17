use std::{marker::PhantomData, str::FromStr};

use actix_web::{
    HttpResponse, Responder, Scope,
    web::{self, Data, Json},
};
use apalis_core::backend::{
    self, Backend, ConfigExt, FetchById, Filter, ListAllTasks, ListQueues, ListTasks, ListWorkers,
    Metrics, TaskSink, codec::Codec, queue,
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::RwLock;

use crate::{
    builder::Builder,
    fetch_queues,
    framework::{ApiBuilder, RegisterRoute},
    get_all_tasks, get_all_workers, get_task_by_id, get_tasks, get_workers, overview, push_task,
    stats_by_queue,
};

pub struct Handler<S, T, Compact> {
    _phantom: PhantomData<(S, T, Compact)>,
}

impl<S, T, Compact> Handler<S, T, Compact> {
    pub async fn get_tasks(
        queue: web::Data<String>,
        storage: web::Data<RwLock<S>>,
        query: web::Query<Filter>,
    ) -> impl Responder
    where
        T: Serialize + DeserializeOwned + 'static,
        S: ListTasks<T> + Send + 'static,
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

    pub async fn push_task(
        queue: web::Data<String>,
        task: Json<T>,
        storage: Data<RwLock<S>>,
    ) -> impl Responder
    where
        T: Serialize + DeserializeOwned + 'static,
        S: TaskSink<T> + Send,
        S::Error: std::error::Error,
        S::Codec: Codec<T, Compact = Compact>,
        <<S as Backend>::Codec as Codec<T>>::Error: std::error::Error,
    {
        let queue = queue.into_inner().to_string();
        match push_task(queue, task.into_inner(), storage.into_inner()).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

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
        let task_id = task_id.into_inner().to_string();
        let storage = storage.into_inner();

        match get_task_by_id::<S, T>(task_id, storage).await {
            Ok(Some(task)) => HttpResponse::Ok().json(task),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

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
        <<S as Backend>::Codec as Codec<<S as Backend>::Args>>::Error: std::error::Error,
    {
        let storage = storage.into_inner();
        let filter = query.into_inner();

        match get_all_tasks::<S>(storage, filter).await {
            Ok(tasks) => HttpResponse::Ok().json(tasks),
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }

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
    <<B as Backend>::Codec as Codec<<B as Backend>::Args>>::Error: std::error::Error,
    T: Serialize + DeserializeOwned + 'static,
    B: ListTasks<T> + FetchById<T>,
    B::Codec: Codec<T, Compact = Compact>,
    <<B as Backend>::Codec as Codec<T>>::Error: std::error::Error,
    B: TaskSink<T> + ConfigExt,
{
    fn register(mut self, backend: B) -> Self {
        let queue = backend.get_queue();
        let backend = web::Data::new(RwLock::new(backend));
        if self.root {
            self.router = self
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
                )
                .route("/events", web::get().to(sse::new_client));
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

pub mod sse {
    use std::{sync::Arc, time::Duration};

    use crate::sse::Broadcaster;
    use actix_web::web::*;
    use actix_web_lab::sse::Event;
    use futures::StreamExt;
    use std::sync::Mutex;

    pub async fn new_client(
        broadcaster: Data<Arc<Mutex<Broadcaster>>>,
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
