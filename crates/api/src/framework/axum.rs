use apalis_board_types::ApiError;
use apalis_core::{
    backend::{
        self, Backend, ConfigExt, FetchById, Filter, ListAllTasks, ListQueues, ListTasks,
        ListWorkers, Metrics, QueueInfo, RunningWorker, Statistic, TaskSink, codec::Codec, queue,
    },
    task::Task,
};
use axum::{
    Extension, Json, Router,
    extract::{FromRequest, Path, Query, Request, rejection::JsonRejection},
    handler::Handler,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, put},
};
use serde::{Serialize, de::DeserializeOwned};
use std::{f64::consts::E, marker::PhantomData, str::FromStr, sync::Arc};
use tokio::sync::RwLock;

use crate::framework::{ApiBuilder, RegisterRoute};

#[derive(Debug)]
pub enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),

    /// An error occurred in the API
    ApiError(ApiError),
    /// Resource not found
    NotFound,

    /// Missing application state
    MissingState,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let response = match self {
            AppError::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (rejection.status(), rejection.body_text()).into_response()
            }
            AppError::ApiError(err) => {
                // These errors are unexpected and should be logged
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err).into_response()).into_response()
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, ()).into_response(),
            AppError::MissingState => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing application state",
            )
                .into_response(),
        };
        response
    }
}

// Some shared state used throughout our application
pub type State<B> = Extension<Arc<RwLock<B>>>;

pub async fn get_tasks<S, T, Compact>(
    query: Query<Filter>,
    queue: Extension<String>,
    storage: State<S>,
) -> Result<Json<Vec<Task<T, S::Context, S::IdType>>>, AppError>
where
    T: Serialize + DeserializeOwned + 'static,
    S: ListTasks<T> + Send + 'static,
    S::Context: Serialize + 'static,
    S::IdType: Serialize + 'static,
    <S as Backend>::Error: std::error::Error + 'static,
    S::Codec: Codec<T, Compact = Compact> + 'static,
    Compact: 'static,
{
    let queue = queue.0.to_string();
    let storage = storage.0;
    let filter = query.0;

    crate::get_tasks::<S, T, Compact>(queue, storage, filter)
        .await
        .map(Json)
        .map_err(AppError::ApiError)
}
pub async fn stats_by_queue<S>(
    queue: Extension<String>,
    storage: State<S>,
) -> Result<Json<Vec<Statistic>>, AppError>
where
    S::Error: std::error::Error,
    S: Metrics,
{
    let queue = queue.0;
    let storage = storage.0;

    match crate::stats_by_queue::<S>(storage, queue.to_string()).await {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

pub async fn get_workers<S>(
    queue: Extension<String>,
    storage: State<S>,
) -> Result<Json<Vec<RunningWorker>>, AppError>
where
    S: ListWorkers,
    S::Error: std::error::Error,
{
    let queue = queue.0.to_string();
    let storage = storage.0;

    match crate::get_workers::<S>(storage, queue).await {
        Ok(workers) => Ok(Json(workers)),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

pub async fn push_task<S, T, Compact>(
    queue: Extension<String>,
    storage: State<S>,
    task: Json<T>,
) -> Result<Json<()>, AppError>
where
    T: Serialize + DeserializeOwned + 'static + Send,
    S: TaskSink<T> + 'static + Send,
    S::Error: std::error::Error,
    S::Codec: Codec<T, Compact = Compact>,
    <<S as Backend>::Codec as Codec<T>>::Error: std::error::Error,
{
    let queue = queue.to_string();
    match crate::push_task(queue, task.0, storage.0).await {
        Ok(_) => Ok(Json(())),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

pub async fn get_task_by_id<S, T>(
    Path(task_id): Path<String>,
    storage: State<S>,
) -> Result<Json<Task<T, S::Context, S::IdType>>, AppError>
where
    T: Serialize + DeserializeOwned + 'static + Send,
    S: FetchById<T> + Send + 'static,
    S::Context: Serialize + 'static + Send,
    S::IdType: Serialize + 'static + Send,
    S::Error: std::error::Error,
    S::IdType: FromStr + 'static + Send,
    <<S as Backend>::IdType as FromStr>::Err: std::error::Error,
{
    let task_id = task_id.to_string();
    let storage = storage.0;

    match crate::get_task_by_id::<S, T>(task_id, storage).await {
        Ok(Some(task)) => Ok(Json(task)),
        Ok(None) => Err(AppError::NotFound),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

pub async fn get_all_tasks<S>(
    query: Query<Filter>,
    storage: State<S>,
) -> Result<Json<Vec<Task<S::Compact, S::Context, S::IdType>>>, AppError>
where
    S: ListAllTasks + Send + 'static,
    S::Context: Serialize,
    S::IdType: Serialize,
    S::Compact: Serialize,
    <S as Backend>::Error: std::error::Error,
    <<S as Backend>::Codec as Codec<<S as Backend>::Args>>::Error: std::error::Error,
{
    let storage = storage.0;
    let filter = query.0;

    match crate::get_all_tasks::<S>(storage, filter).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(e) => Err(AppError::ApiError(e)),
    }
}

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
    B: Metrics + ListWorkers + ListAllTasks + ListQueues,
    B::Context: Serialize,
    B::IdType: Serialize,
    <B as Backend>::Error: std::error::Error,
    B::IdType: FromStr + 'static + Send,
    <<B as Backend>::IdType as FromStr>::Err: std::error::Error,
    Compact: Serialize + 'static + Send,
    B::Compact: Serialize + 'static + Send,
    B::Context: Serialize + 'static + Send,
    <B as Backend>::Error: std::error::Error,
    <<B as Backend>::Codec as Codec<<B as Backend>::Args>>::Error: std::error::Error,
    T: Serialize + DeserializeOwned + 'static + Send,
    B: ListTasks<T> + FetchById<T>,
    B::Codec: Codec<T, Compact = Compact>,
    <<B as Backend>::Codec as Codec<T>>::Error: std::error::Error,
    B: TaskSink<T> + ConfigExt + Send + Sync + 'static,
{
    fn register(mut self, backend: B) -> Self {
        let queue = backend.get_queue();
        let backend = Arc::new(RwLock::new(backend));
        if self.root {
            self.router = self
                .router
                .route("/", get(fetch_queues::<B>))
                .route("/tasks", get(get_all_tasks::<B>))
                .route("/workers", get(get_all_workers::<B>))
                .route("/overview", get(overview::<B>))
                .route("/events", get(sse::new_client))
                .layer(Extension(backend.clone()));
        }
        let scope = self.router.nest(
            &format!("/queues/{queue}"),
            Router::new()
                .route("/tasks", get(get_tasks::<B, T, Compact>))
                .route("/stats", get(stats_by_queue::<B>))
                .route("/workers", get(get_workers::<B>))
                .route("/tasks", put(push_task::<B, T, Compact>))
                .route("/tasks/{task_id}", get(get_task_by_id::<B, T>))
                .layer(Extension(queue.to_string()))
                .layer(Extension(backend.clone())),
        );

        Self {
            router: scope,
            root: false,
        }
    }
}

pub mod sse {

    use std::{sync::Mutex, time::Duration};

    use axum::response::{Sse, sse::Event};
    use futures::{Stream, StreamExt, channel::mpsc::TryRecvError};

    use crate::sse::Broadcaster;

    use super::*;
    pub async fn new_client(
        broadcaster: Extension<Arc<Mutex<Broadcaster>>>,
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
