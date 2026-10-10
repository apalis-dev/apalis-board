use apalis_board_types::ApiError;
use apalis_core::{
    backend::{
        Backend, BackendConfig, FetchById, Filter, ListAllTasks, ListQueues, ListTasks,
        ListWorkers, Metrics, QueueInfo, RunningWorker, Statistic, TaskSink, WireFormatBackend,
        codec::Codec, queue::Queue,
    },
    task::{Task, builder::TaskBuilder, metadata::MetadataStore},
};
use axum::{
    Extension, Json, Router,
    body::Body,
    extract::{FromRequest, Path, Query, rejection::JsonRejection},
    http::{HeaderMap, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, put},
};

use serde::{Serialize, de::DeserializeOwned};
use std::{
    fmt::Display,
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;

use crate::{
    META_PREFIX, RawTask,
    framework::{ApiBuilder, RegisterRoute, TaskRejection},
};

/// An enumeration of possible application errors.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// The request body contained invalid JSON
    #[error("JSON Rejection: {0}")]
    JsonRejection(JsonRejection),

    /// An error occurred in the API
    #[error("API Error: {0}")]
    ApiError(ApiError),
    /// Resource not found
    #[error("Resource not found")]
    NotFound,

    /// Missing application state
    #[error("Missing application state")]
    MissingState,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (rejection.status(), rejection.body_text()).into_response()
            }
            Self::ApiError(err) => {
                // These errors are unexpected and should be logged
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err).into_response()).into_response()
            }
            Self::NotFound => (StatusCode::NOT_FOUND, ()).into_response(),
            Self::MissingState => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing application state",
            )
                .into_response(),
        }
    }
}

/// Type alias for application state extension.
pub type State<B> = Extension<Arc<RwLock<B>>>;

/// Fetch all tasks from the backend storage.
pub async fn get_tasks<S, T>(
    query: Query<Filter>,
    storage: State<S>,
) -> Result<Json<Vec<RawTask<T>>>, AppError>
where
    S: ListTasks + Send + 'static,
    <S as Backend>::Error: std::error::Error + 'static,
    S::Compact: Serialize + 'static,
    S::Codec: Codec<T, Compact = S::Compact>,
    T: Serialize,
    <S::Codec as Codec<T>>::Error: std::error::Error + 'static,
{
    let storage = storage.0;
    let filter = query.0;

    crate::get_tasks::<S, T>(storage, filter)
        .await
        .map(|tasks| Json(tasks.into_iter().map(RawTask).collect()))
        .map_err(AppError::ApiError)
}

/// Fetch statistics for a specific queue from the backend storage.
pub async fn stats_by_queue<S>(storage: State<S>) -> Result<Json<Vec<Statistic>>, AppError>
where
    S::Error: std::error::Error,
    S: Metrics,
{
    let storage = storage.0;

    match crate::stats_by_queue::<S>(storage).await {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

/// Fetch all workers from the backend storage.
pub async fn get_workers<S>(storage: State<S>) -> Result<Json<Vec<RunningWorker>>, AppError>
where
    S: ListWorkers,
    S::Error: std::error::Error,
{
    let storage = storage.0;

    match crate::get_workers::<S>(storage).await {
        Ok(workers) => Ok(Json(workers)),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

/// Push a new task to the backend storage.
pub async fn push_task<S, T>(storage: State<S>, task: RawTask<T>) -> Result<Json<()>, AppError>
where
    S: BackendConfig,
    T: Serialize + DeserializeOwned + 'static + Send,
    S: TaskSink<T, S::Kind> + 'static + Send,
    S::Error: std::error::Error,
{
    match crate::push_task(task, storage.0).await {
        Ok(_) => Ok(Json(())),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

/// Fetch a task by its ID from the backend storage.
pub async fn get_task_by_id<S, T>(
    Path(task_id): Path<String>,
    storage: State<S>,
) -> Result<Json<RawTask<T>>, AppError>
where
    S: FetchById + WireFormatBackend + Backend<Task = Task<S::Compact>> + Send + 'static,
    S::Error: std::error::Error,
    T: Serialize + 'static + Send,
    S::Codec: Codec<T, Compact = S::Compact>,
    <S::Codec as Codec<T>>::Error: std::error::Error,
{
    let task_id = task_id.clone();
    let storage = storage.0;

    match crate::get_task_by_id::<S, T>(task_id, storage).await {
        Ok(Some(task)) => Ok(Json(RawTask(task))),
        Ok(None) => Err(AppError::NotFound),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

/// Fetch all tasks from the backend storage.
pub async fn get_all_tasks<S>(
    query: Query<Filter>,
    storage: State<S>,
) -> Result<Json<Vec<RawTask<S::Compact>>>, AppError>
where
    S: ListAllTasks + Send + 'static,
    S::Compact: Serialize,
{
    let storage = storage.0;
    let filter = query.0;

    match crate::get_all_tasks::<S>(storage, filter).await {
        Ok(tasks) => Ok(Json(tasks.into_iter().map(RawTask).collect())),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

/// Fetch all workers from the backend storage.
pub async fn get_all_workers<S>(storage: State<S>) -> Result<Json<Vec<RunningWorker>>, AppError>
where
    S: ListWorkers + 'static,
    S::Error: std::error::Error,
{
    let storage = storage.0;

    match crate::get_all_workers::<S>(storage).await {
        Ok(workers) => Ok(Json(workers)),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

/// Fetch all queues from the backend storage.
pub async fn fetch_queues<S>(storage: State<S>) -> Result<Json<Vec<QueueInfo>>, AppError>
where
    S::Error: std::error::Error,
    S: ListQueues,
{
    let storage = storage.0;

    crate::fetch_queues::<S>(storage)
        .await
        .map_err(AppError::ApiError)
        .map(Json)
}

/// Get an overview of statistics across all queues.
pub async fn overview<S>(storage: State<S>) -> Result<Json<Vec<Statistic>>, AppError>
where
    S::Error: std::error::Error,
    S: Metrics + 'static,
{
    let storage = storage.0;

    let tasks = crate::overview::<S>(storage)
        .await
        .map_err(AppError::ApiError)?;

    Ok(Json(tasks))
}

impl<B, T, Compact> RegisterRoute<B, T> for ApiBuilder<Router>
where
    B: Metrics + ListWorkers + ListAllTasks + ListQueues + BackendConfig + Send + 'static,
    <B as Backend>::Error: std::error::Error,
    Compact: Serialize + 'static,
    B::Compact: Serialize,
    <B as Backend>::Error: std::error::Error,
    T: Serialize + DeserializeOwned + Send + 'static,
    B: ListTasks + FetchById,
    B::Codec: Codec<T, Compact = Compact>,
    B: TaskSink<T, B::Kind> + Send + Sync + 'static,
    B: WireFormatBackend<Compact = Compact> + Backend<Task = Task<Compact>>,
    <B::Codec as Codec<T>>::Error: std::error::Error,
{
    fn register(mut self, queue: &str, backend: B) -> Self {
        let queue = Queue::from_str(queue).unwrap();
        let backend = Arc::new(RwLock::new(backend));
        if self.root {
            #[allow(unused_mut)]
            let mut r = self
                .router
                .route("/queues", get(fetch_queues::<B>))
                .route("/tasks", get(get_all_tasks::<B>))
                .route("/workers", get(get_all_workers::<B>))
                .route("/overview", get(overview::<B>));

            #[cfg(feature = "sse")]
            {
                r = r.route("/events", get(sse::new_client));
            }

            self.router = r.layer(Extension(backend.clone()));
        }
        let scope = self.router.nest(
            &format!("/queues/{queue}"),
            Router::new()
                .route("/tasks", get(get_tasks::<B, T>))
                .route("/stats", get(stats_by_queue::<B>))
                .route("/workers", get(get_workers::<B>))
                .route("/tasks", put(push_task::<B, T>))
                .route("/tasks/{task_id}", get(get_task_by_id::<B, T>))
                .layer(Extension(queue))
                .layer(Extension(backend)),
        );

        Self {
            router: scope,
            root: false,
        }
    }
}

#[cfg(feature = "ui")]
mod ui {
    use std::{
        convert::Infallible,
        task::{Context, Poll},
    };

    use apalis_core::layers::Service;
    use axum::{
        body::Body,
        http::{Request, Response, StatusCode},
    };

    use crate::ui::ServeUI;

    impl Service<Request<Body>> for ServeUI {
        type Response = Response<Body>;
        type Error = Infallible;
        type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: Request<Body>) -> Self::Future {
            let path = req.uri().path();
            let mut file = Self::get_file(path);

            // If no matching file, fall back to index.html
            if file.is_none() {
                file = Self::get_file("index.html");
            }

            let response = match file {
                Some(file) => {
                    let path_str = file.path().to_str().unwrap_or("");
                    let content_type = Self::content_type(path_str);
                    let mut builder = Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", content_type);

                    if let Some(cache) = Self::cache_control(path_str) {
                        builder = builder.header("Cache-Control", cache);
                    }

                    builder.body(file.contents().to_vec().into()).unwrap()
                }
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Vec::new().into())
                    .unwrap(),
            };

            std::future::ready(Ok(response))
        }
    }
}

/// Expose Server-Sent Events (SSE) functionality.
#[cfg(feature = "sse")]
pub mod sse {

    use std::{sync::Mutex, time::Duration};

    use axum::response::{Sse, sse::Event};
    use futures::{Stream, StreamExt, channel::mpsc::TryRecvError};

    use crate::sse::TracingBroadcaster;

    use super::*;

    /// Create a new SSE client and register it with the broadcaster.
    pub async fn new_client(
        broadcaster: Extension<Arc<Mutex<TracingBroadcaster>>>,
    ) -> Sse<impl Stream<Item = Result<Event, TryRecvError>>> {
        let rx = broadcaster.lock().unwrap().new_client();
        let stream = rx
            .filter(|s| futures::future::ready(s.as_ref().is_ok_and(|e| e.span.is_some())))
            .map(|entry| Ok(Event::default().json_data(entry?).unwrap()));

        Sse::new(stream).keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(1))
                .text("keep-alive-text"),
        )
    }
}

impl<S, T> FromRequest<S> for RawTask<T>
where
    S: Send + Sync,
    T: DeserializeOwned + 'static,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let task = extract_task(req.headers())
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

        let Json(args) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

        Ok(Self(task.map_args(|_| args)))
    }
}

fn extract_task(headers: &HeaderMap) -> Result<Task<()>, TaskRejection> {
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
