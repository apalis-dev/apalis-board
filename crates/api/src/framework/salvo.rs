use apalis_core::{
    backend::{
        Backend, BackendConfig, FetchById, Filter, ListAllTasks, ListQueues, ListTasks,
        ListWorkers, Metrics, QueueInfo, RunningWorker, Statistic, TaskSink, WireFormatBackend,
        codec::Codec,
    },
    task::{Task, builder::TaskBuilder, metadata::MetadataStore},
};
use http::HeaderMap;
use salvo::{affix_state, prelude::*};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fmt::Display,
    marker::PhantomData,
    time::{SystemTime, UNIX_EPOCH},
};
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    META_PREFIX, RawTask,
    framework::{ApiBuilder, RegisterRoute, TaskRejection},
};

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
    async fn handle(&self, depot: &Depot) -> Result<Json<Vec<QueueInfo>>, StatusError> {
        let state = depot
            .get_typed::<ApiState<B>>()
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
    B: ListAllTasks + Send + Sync + 'static,
    B::Compact: Serialize + Send,
{
    async fn handle(
        &self,
        req: &mut Request,
        depot: &Depot,
    ) -> Result<Json<Vec<RawTask<B::Compact>>>, StatusError> {
        let state = depot
            .get_typed::<ApiState<B>>()
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

        Ok(Json(tasks.into_iter().map(RawTask).collect()))
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
    async fn handle(&self, depot: &Depot) -> Result<Json<Vec<RunningWorker>>, StatusError> {
        let state = depot
            .get_typed::<ApiState<B>>()
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
    async fn handle(&self, depot: &Depot) -> Result<Json<Vec<Statistic>>, StatusError> {
        let state = depot
            .get_typed::<ApiState<B>>()
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
    B: ListTasks + WireFormatBackend<Compact = Compact> + Send + Sync + 'static,
    <B as Backend>::Error: std::error::Error + 'static,
    B::Codec: Codec<T, Compact = Compact> + 'static,
    Compact: 'static,
    <B::Codec as Codec<T>>::Error: std::error::Error + 'static,
{
    async fn handle(
        &self,
        req: &mut Request,
        depot: &Depot,
    ) -> Result<Json<Vec<RawTask<T>>>, StatusError> {
        let state = depot
            .get_typed::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;
        let filter = req
            .parse_queries::<Filter>()
            .map_err(|err| StatusError::bad_request().brief(err.to_string()))?;

        let backend = state.backend.read().await;
        let tasks = backend.list_tasks(&filter).await.map_err(internal_error)?;

        Ok(Json(
            tasks
                .into_iter()
                .map(|task| {
                    task.try_map_args(|compact| {
                        backend
                            .codec()
                            .decode(&compact)
                            .map_err(|e| StatusError::internal_server_error().brief(e.to_string()))
                    })
                    .map(RawTask)
                })
                .collect::<Result<Vec<RawTask<T>>, StatusError>>()?,
        ))
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
    B: Metrics + Send + Sync + 'static,
    B::Error: std::error::Error,
{
    async fn handle(&self, depot: &Depot) -> Result<Json<Vec<Statistic>>, StatusError> {
        let state = depot
            .get_typed::<ApiState<B>>()
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
    B: ListWorkers + Send + Sync + 'static,
    B::Error: std::error::Error,
{
    async fn handle(&self, depot: &Depot) -> Result<Json<Vec<RunningWorker>>, StatusError> {
        let state = depot
            .get_typed::<ApiState<B>>()
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
    B: WireFormatBackend + BackendConfig + TaskSink<T, B::Kind> + Send + Sync + 'static,
    B::Error: std::error::Error,
    B::Codec: Codec<T, Compact = Compact>,
    Compact: Send + 'static,
    <<B as WireFormatBackend>::Codec as Codec<T>>::Error: std::error::Error,
{
    async fn handle(&self, req: &mut Request, depot: &Depot) -> Result<Json<()>, StatusError> {
        let state = depot
            .get_typed::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;
        let task =
            extract_task(req).map_err(|e| StatusError::bad_request().brief(e.to_string()))?;

        let args = req
            .parse_body::<T>()
            .await
            .map_err(|err| StatusError::bad_request().brief(err.to_string()))?;
        let task = task.map_args(|_| args);

        crate::push_task(RawTask(task), state.backend.clone())
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
    B: FetchById + WireFormatBackend + Send + Sync + 'static,
    B::Error: std::error::Error,
    B: Backend<Task = Task<B::Compact>>,
    B::Codec: Codec<T, Compact = B::Compact>,
    <B::Codec as Codec<T>>::Error: std::error::Error,
{
    async fn handle(&self, req: &Request, depot: &Depot) -> Result<Json<RawTask<T>>, StatusError> {
        let state = depot
            .get_typed::<ApiState<B>>()
            .map_err(|_| StatusError::internal_server_error())?;
        let task_id = req
            .param::<String>("task_id")
            .ok_or_else(|| StatusError::bad_request().brief("missing task_id"))?;

        let task = crate::get_task_by_id::<B, T>(task_id, state.backend.clone())
            .await
            .map_err(internal_error)?;

        match task {
            Some(task) => Ok(Json(RawTask(task))),
            None => Err(StatusError::not_found()),
        }
    }
}

impl<B, T, Compact> RegisterRoute<B, T> for ApiBuilder<Router>
where
    B: Metrics + ListWorkers + ListAllTasks + ListQueues + BackendConfig + Send + 'static,
    <B as Backend>::Error: std::error::Error,
    Compact: Serialize + 'static,
    B::Compact: Serialize + Send,
    <B as Backend>::Error: std::error::Error,
    T: Serialize + DeserializeOwned + Send + 'static,
    B: ListTasks + FetchById,
    B::Codec: Codec<T, Compact = Compact>,
    B: TaskSink<T, B::Kind> + Send + Sync + 'static,
    B: WireFormatBackend<Compact = Compact> + Backend<Task = Task<Compact>>,
    <B::Codec as Codec<T>>::Error: std::error::Error,
{
    fn register(mut self, queue: &str, backend: B) -> Self {
        let backend = Arc::new(RwLock::new(backend));
        let state = ApiState { backend };

        if self.root {
            let mut router = self
                .router
                .hoop(affix_state::inject(state.clone()))
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

        let queue_router = Router::with_path(format!("/queues/{queue}"))
            .hoop(affix_state::inject(state))
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

    /// A structure to server the route endpoint of dashboard
    #[derive(Debug, Default)]
    pub struct ServeApp;

    impl ServeApp {
        /// Creates a new instance of `ServeApp`.
        #[must_use]
        pub fn new() -> Self {
            Self
        }
        /// associate method to return the router of endpoint dashboard
        #[must_use]
        pub fn router() -> Router {
            Router::with_path("{*path}").get(ServeUI::new())
        }
    }

    #[handler]
    impl ServeUI {
        async fn handle(&self, req: &Request, res: &mut Response) -> Result<(), StatusError> {
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

/// SSE setup for Salvo
#[cfg(feature = "sse")]
pub mod sse {
    use std::io;
    use std::sync::{Arc, Mutex};

    use futures::StreamExt;
    use salvo::prelude::*;
    use salvo::sse::{SseEvent, SseKeepAlive};

    use crate::sse::TracingBroadcaster;

    /// handle for endpoint /events
    #[handler]
    pub async fn new_client(depot: &Depot, res: &mut Response) {
        let Ok(broadcaster) = depot.get_typed::<Arc<Mutex<TracingBroadcaster>>>() else {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        };

        let rx = broadcaster.lock().unwrap().new_client();

        let stream = rx
            .filter(|s| futures::future::ready(s.as_ref().is_ok_and(|e| e.span.is_some())))
            .map(|entry| -> Result<SseEvent, io::Error> {
                let entry = entry.map_err(io::Error::other)?;
                Ok(SseEvent::default().json(entry)?)
            });

        SseKeepAlive::new(stream).stream(res);
    }
}

fn extract_task(req: &Request) -> Result<Task<()>, TaskRejection> {
    let headers = req.headers();
    let mut task = TaskBuilder::new(());

    if let Some(id) = parse_header(headers, "x-task-id")? {
        task = task.task_id(id);
    }

    if let Some(attempt) = parse_header::<usize>(headers, "x-task-attempt")? {
        task = task.attempt(attempt);
    }

    if let Some(status) = parse_header(headers, "x-task-status")? {
        task = task.status(status);
    }

    if let Some(run_at) = parse_header(headers, "x-task-run-at")? {
        task = task.run_at_timestamp(run_at);
    } else {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        task = task.run_at_timestamp(now.as_secs())
    }

    task = task.done_at(parse_header(headers, "x-task-done-at")?);

    task = task.lock_at(parse_header(headers, "x-task-lock-at")?);

    task = task.lock_by(parse_header(headers, "x-task-lock-by")?);

    if let Some(idempotency_key) = parse_header::<String>(headers, "x-task-idempotency-key")? {
        task = task.idempotency_key(idempotency_key);
    }

    if let Some(max_attempts) = parse_header(headers, "x-task-max-attempts")? {
        task = task.max_attempts(max_attempts);
    }

    if let Some(priority) = parse_header(headers, "x-task-priority")? {
        task = task.priority(priority);
    }

    let mut metadata = MetadataStore::default();
    for (name, value) in headers.iter() {
        if let Some(key) = name.as_str().strip_prefix(META_PREFIX) {
            let value = value.to_str().map_err(|e| TaskRejection::BadHeader {
                name: name.to_string(),
                reason: e.to_string(),
            })?;

            metadata.insert(key.to_owned(), value.to_owned()).unwrap();
        }
    }

    task = task.with_metadata(metadata);

    Ok(task.build())
}

fn parse_header<T>(headers: &HeaderMap, name: &str) -> Result<Option<T>, TaskRejection>
where
    T: FromStr,
    T::Err: Display,
{
    let Some(raw) = headers.get(name) else {
        return Ok(None);
    };
    let bad = |reason: String| TaskRejection::BadHeader {
        name: name.into(),
        reason,
    };
    let s = raw.to_str().map_err(|e| bad(e.to_string()))?;
    s.parse::<T>().map(Some).map_err(|e| bad(e.to_string()))
}
