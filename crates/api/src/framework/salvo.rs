use apalis_core::{
    backend::{
        Backend, BackendExt, FetchById, Filter, ListAllTasks, ListQueues, ListTasks, ListWorkers,
        Metrics, QueueInfo, RunningWorker, Statistic, TaskSink, codec::Codec,
    },
    task::Task,
};
use salvo::{affix_state, prelude::*};
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

use crate::framework::{ApiBuilder, RegisterRoute};

/// State to inject in Depot during the request in order to access to Api Backend
#[derive(Debug)]
pub struct ApiState<B> {
    /// field to store state backend
    pub backend: Arc<RwLock<B>>,
}

impl<B> Clone for ApiState<B> {
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone(),
        }
    }
}

fn internal_error<E: std::fmt::Display>(err: E) -> StatusError {
    StatusError::internal_server_error().brief(err.to_string())
}

struct FetchQueues<B>(PhantomData<fn() -> B>);

impl<B> FetchQueues<B> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

#[handler]
impl<B> FetchQueues<B>
where
    B: ListQueues + Send + Sync + 'static,
    B::Error: std::error::Error,
{
    async fn handle(&self, depot: &mut Depot) -> Result<Json<Vec<QueueInfo>>, StatusError> {
        let state = depot
            .obtain::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;

        let queues = state
            .backend
            .read()
            .await
            .list_queues()
            .await
            .map_err(internal_error)?;

        Ok(Json(queues))
    }
}

struct GetAllTasks<B>(PhantomData<fn() -> B>);

impl<B> GetAllTasks<B> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

#[handler]
impl<B> GetAllTasks<B>
where
    B: ListAllTasks + BackendExt + Send + Sync + 'static,
    B::Context: Serialize + Send,
    B::IdType: Serialize + Send,
    B::Compact: Serialize + Send,
    <B as Backend>::Error: std::error::Error,
    <<B as BackendExt>::Codec as Codec<<B as Backend>::Args>>::Error: std::error::Error,
{
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
    ) -> Result<Json<Vec<Task<B::Compact, B::Context, B::IdType>>>, StatusError> {
        let state = depot
            .obtain::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;
        let filter = req
            .parse_queries::<Filter>()
            .map_err(|err| StatusError::bad_request().brief(err.to_string()))?;

        let tasks = state
            .backend
            .read()
            .await
            .list_all_tasks(&filter)
            .await
            .map_err(internal_error)?;

        Ok(Json(tasks))
    }
}

struct GetAllWorkers<B>(PhantomData<fn() -> B>);

impl<B> GetAllWorkers<B> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

#[handler]
impl<B> GetAllWorkers<B>
where
    B: ListWorkers + Send + Sync + 'static,
    B::Error: std::error::Error,
{
    async fn handle(&self, depot: &mut Depot) -> Result<Json<Vec<RunningWorker>>, StatusError> {
        let state = depot
            .obtain::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;

        let workers = state
            .backend
            .read()
            .await
            .list_all_workers()
            .await
            .map_err(internal_error)?;

        Ok(Json(workers))
    }
}

struct Overview<B>(PhantomData<fn() -> B>);

impl<B> Overview<B> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

#[handler]
impl<B> Overview<B>
where
    B: Metrics + Send + Sync + 'static,
    B::Error: std::error::Error,
{
    async fn handle(&self, depot: &mut Depot) -> Result<Json<Vec<Statistic>>, StatusError> {
        let state = depot
            .obtain::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;

        let stats = state
            .backend
            .read()
            .await
            .global()
            .await
            .map_err(internal_error)?;

        Ok(Json(stats))
    }
}

struct GetTasks<B, T, Compact>(
    PhantomData<fn() -> B>,
    PhantomData<fn() -> T>,
    PhantomData<fn() -> Compact>,
);

impl<B, T, Compact> GetTasks<B, T, Compact> {
    fn new() -> Self {
        Self(PhantomData, PhantomData, PhantomData)
    }
}

#[handler]
impl<B, T, Compact> GetTasks<B, T, Compact>
where
    T: Serialize + DeserializeOwned + Send + 'static,
    B: ListTasks<T> + BackendExt + Send + Sync + 'static,
    B::Context: Serialize + Send + 'static,
    B::IdType: Serialize + Send + 'static,
    <B as Backend>::Error: std::error::Error + 'static,
    B::Codec: Codec<T, Compact = Compact> + 'static,
    Compact: 'static,
{
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
    ) -> Result<Json<Vec<Task<T, B::Context, B::IdType>>>, StatusError> {
        let state = depot
            .obtain::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;
        let filter = req
            .parse_queries::<Filter>()
            .map_err(|err| StatusError::bad_request().brief(err.to_string()))?;

        let tasks = state
            .backend
            .read()
            .await
            .list_tasks(&filter)
            .await
            .map_err(internal_error)?;

        Ok(Json(tasks))
    }
}

struct StatsByQueue<B>(PhantomData<fn() -> B>);

impl<B> StatsByQueue<B> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

#[handler]
impl<B> StatsByQueue<B>
where
    B: Metrics + BackendExt + Send + Sync + 'static,
    B::Error: std::error::Error,
{
    async fn handle(&self, depot: &mut Depot) -> Result<Json<Vec<Statistic>>, StatusError> {
        let state = depot
            .obtain::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;

        let stats = state
            .backend
            .read()
            .await
            .fetch_by_queue()
            .await
            .map_err(internal_error)?;

        Ok(Json(stats))
    }
}

struct GetWorkers<B>(PhantomData<fn() -> B>);

impl<B> GetWorkers<B> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

#[handler]
impl<B> GetWorkers<B>
where
    B: ListWorkers + BackendExt + Send + Sync + 'static,
    B::Error: std::error::Error,
{
    async fn handle(&self, depot: &mut Depot) -> Result<Json<Vec<RunningWorker>>, StatusError> {
        let state = depot
            .obtain::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;

        let workers = state
            .backend
            .read()
            .await
            .list_workers()
            .await
            .map_err(internal_error)?;

        Ok(Json(workers))
    }
}

struct PushTask<B, T, Compact>(
    PhantomData<fn() -> B>,
    PhantomData<fn() -> T>,
    PhantomData<fn() -> Compact>,
);

impl<B, T, Compact> PushTask<B, T, Compact> {
    fn new() -> Self {
        Self(PhantomData, PhantomData, PhantomData)
    }
}

#[handler]
impl<B, T, Compact> PushTask<B, T, Compact>
where
    T: Serialize + DeserializeOwned + Send + 'static,
    B: TaskSink<T> + BackendExt + Send + Sync + 'static,
    B::Error: std::error::Error,
    B::Context: Send + 'static,
    B::IdType: Send + 'static,
    B::Codec: Codec<T, Compact = Compact>,
    Compact: Send + 'static,
    <<B as BackendExt>::Codec as Codec<T>>::Error: std::error::Error,
{
    async fn handle(&self, req: &mut Request, depot: &mut Depot) -> Result<Json<()>, StatusError> {
        let state = depot
            .obtain::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;
        let task = req
            .parse_body::<T>()
            .await
            .map_err(|err| StatusError::bad_request().brief(err.to_string()))?;

        crate::push_task(task, state.backend.clone())
            .await
            .map_err(internal_error)?;

        Ok(Json(()))
    }
}

struct GetTaskById<B, T>(PhantomData<fn() -> B>, PhantomData<fn() -> T>);

impl<B, T> GetTaskById<B, T> {
    fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

#[handler]
impl<B, T> GetTaskById<B, T>
where
    T: Serialize + DeserializeOwned + 'static + Send,
    B: FetchById<T> + Send + Sync + 'static,
    B::Context: Serialize + 'static + Send,
    B::IdType: Serialize + 'static + Send,
    B::Error: std::error::Error,
    B::IdType: FromStr + 'static + Send,
    <<B as Backend>::IdType as FromStr>::Err: std::error::Error,
{
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
    ) -> Result<Json<Task<T, B::Context, B::IdType>>, StatusError> {
        let state = depot
            .obtain::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;
        let task_id = req
            .param::<String>("task_id")
            .ok_or_else(|| StatusError::bad_request().brief("missing task_id"))?;

        let task = crate::get_task_by_id::<B, T>(task_id, state.backend.clone())
            .await
            .map_err(internal_error)?;

        match task {
            Some(task) => Ok(Json(task)),
            None => Err(StatusError::not_found()),
        }
    }
}

impl<B, T, Compact> RegisterRoute<B, T> for ApiBuilder<Router>
where
    B: Metrics + ListWorkers + ListAllTasks + ListQueues + Send + Sync + 'static,
    B::Context: Serialize,
    B::IdType: Serialize,
    <B as Backend>::Error: std::error::Error + 'static,
    B::IdType: FromStr + 'static + Send,
    <<B as Backend>::IdType as FromStr>::Err: std::error::Error,
    Compact: Serialize + 'static + Send,
    B::Compact: Serialize + 'static + Send,
    B::Context: Serialize + 'static + Send,
    <<B as BackendExt>::Codec as Codec<<B as Backend>::Args>>::Error: std::error::Error,
    T: Serialize + DeserializeOwned + 'static + Send,
    B: ListTasks<T> + FetchById<T>,
    B::Codec: Codec<T, Compact = Compact> + 'static,
    <<B as BackendExt>::Codec as Codec<T>>::Error: std::error::Error,
    B: TaskSink<T> + BackendExt,
{
    fn register(mut self, backend: B) -> Self {
        let queue = backend.get_queue();
        let backend = Arc::new(RwLock::new(backend));
        let state = ApiState {
            backend: backend.clone(),
        };

        if self.root {
            let mut router = self
                .router
                .hoop(affix_state::inject(state.clone()))
                .hoop(affix_state::inject(queue.clone()))
                .push(Router::with_path("/queues").get(FetchQueues::<B>::new()))
                .push(Router::with_path("/tasks").get(GetAllTasks::<B>::new()))
                .push(Router::with_path("/workers").get(GetAllWorkers::<B>::new()))
                .push(Router::with_path("/overview").get(Overview::<B>::new()));

            #[cfg(feature = "sse")]
            {
                router = router.push(Router::with_path("/events").get(sse::new_client));
            }

            self.router = router;
        }

        let queue_router = Router::with_path(&format!("/queues/{queue}"))
            .hoop(affix_state::inject(state.clone()))
            .hoop(affix_state::inject(queue.clone()))
            .push(Router::with_path("/tasks").get(GetTasks::<B, T, Compact>::new()))
            .push(Router::with_path("/stats").get(StatsByQueue::<B>::new()))
            .push(Router::with_path("/workers").get(GetWorkers::<B>::new()))
            .push(Router::with_path("/tasks").put(PushTask::<B, T, Compact>::new()))
            .push(Router::with_path("/tasks/{task_id}").get(GetTaskById::<B, T>::new()));

        Self {
            router: self.router.push(queue_router),
            root: false,
        }
    }
}

/// module UI for the bashboard
#[cfg(feature = "ui")]
pub mod ui {
    use crate::ui::ServeUI;
    use salvo::prelude::*;

    /// A structure to server the route endpoint of dasboard
    #[derive(Debug)]
    pub struct ServeApp;

    impl ServeApp {
        /// Creates a new instance of `ServeApp`.
        pub fn new() -> Self {
            Self
        }
        /// associate methode to return the router of endpoint dashboard
        pub fn router() -> Router {
            Router::with_path("{*path}").get(ServeUI::new())
        }
    }

    #[handler]
    impl ServeUI {
        async fn handle(&self, req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
            let path = req.uri().path();
            let mut file = Self::get_file(path);

            // If no matching file, fall back to index.html
            if file.is_none() {
                file = Self::get_file("index.html");
            }

            match file {
                Some(file) => {
                    let path_str = file.path().to_str().unwrap_or("");
                    let content_type = Self::content_type(path_str);
                    res.add_header("Content-Type", content_type, true)
                        .expect("Failed to add header Content-Type");

                    if let Some(cache) = Self::cache_control(path_str) {
                        res.add_header("Cache-Control", cache, true)
                            .expect("fialed to add header Cache-Control");
                    }

                    res.body(file.contents().to_vec());
                }
                None => {
                    res.status_code(StatusCode::NOT_FOUND);
                }
            }

            Ok(())
        }
    }
}

/// module Sever Send Event
/// enable endpoint /events
#[cfg(feature = "sse")]
pub mod sse {
    use std::{
        sync::{Arc, Mutex},
        // time::Duration,
    };

    use futures::{StreamExt, channel::mpsc::TryRecvError};
    use salvo::prelude::*;
    use salvo::sse::{SseEvent, SseKeepAlive};

    use crate::sse::TracingBroadcaster;

    /// handle for endpoint /events
    #[handler]
    pub async fn new_client(depot: &mut Depot, res: &mut Response) {
        let broadcaster = match depot.obtain::<Arc<Mutex<TracingBroadcaster>>>() {
            Ok(b) => b,
            Err(_) => {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                return;
            }
        };

        let rx = broadcaster.lock().unwrap().new_client();

        let stream = rx
            .filter(|s| futures::future::ready(s.as_ref().is_ok_and(|e| e.span.is_some())))
            .map(|entry| -> Result<SseEvent, TryRecvError> {
                let entry = entry?;
                // NOTE: serializing manually here — see caveat below on SseEvent's JSON API.
                let json = serde_json::to_string(&entry).unwrap_or_default();
                Ok(SseEvent::default().json(json).expect("Failed to parse SSE event to JSON"))
            });

        SseKeepAlive::new(stream).stream(res);
    }
}
