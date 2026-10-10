#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
use std::{str::FromStr, sync::Arc};

use apalis_board_types::ApiError;
use apalis_core::{
    backend::{
        Backend, BackendConfig, FetchById, Filter, ListAllTasks, ListQueues, ListTasks,
        ListWorkers, Metrics, QueueInfo, RunningWorker, Statistic, TaskSink, WireFormatBackend,
        codec::Codec,
    },
    task::{ExecutionContext, Task, task_id::TaskId},
};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned, ser::SerializeStruct,
};
use tokio::sync::RwLock;

/// Prefix for task-related headers in HTTP requests.
pub const PREFIX: &str = "x-task-";
/// Prefix for task metadata headers in HTTP requests.
pub const META_PREFIX: &str = "x-task-meta-";

/// Contains different web framework routes.
pub mod framework;
/// Expose Server-Sent Events (SSE) functionality.
#[cfg(feature = "sse")]
pub mod sse;
/// Expose UI components and functionality.
#[cfg(feature = "ui")]
pub mod ui;

/// Push a new task to the specified queue.
pub async fn push_task<Args, B>(
    task: RawTask<Args>,
    storage: Arc<RwLock<B>>,
) -> Result<(), ApiError>
where
    B: BackendConfig,
    Args: Serialize + DeserializeOwned + 'static,
    B: TaskSink<Args, B::Kind> + Send,
{
    let res = storage.write().await.push_task(task.0).await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::BackendError(e.to_string())),
    }
}
/// Get statistics for a specific queue.
pub async fn stats_by_queue<S>(storage: Arc<RwLock<S>>) -> Result<Vec<Statistic>, ApiError>
where
    S::Error: std::error::Error,
    S: Metrics,
{
    let storage = storage.read().await;
    let stats = storage.fetch_by_queue().await;
    match stats {
        Ok(stats) => Ok(stats),
        Err(e) => Err(ApiError::BackendError(e.to_string())),
    }
}

/// Get a list of tasks from the specified queue with filtering options.
pub async fn get_tasks<S, Args>(
    storage: Arc<RwLock<S>>,
    filter: Filter,
) -> Result<Vec<Task<Args>>, ApiError>
where
    S: ListTasks + Send,
    Args: Serialize,
    S::Codec: Codec<Args, Compact = S::Compact>,
    <S::Codec as Codec<Args>>::Error: std::error::Error,
{
    let storage = storage.read().await;

    let tasks = storage.list_tasks(&filter).await;
    let codec = storage.codec();
    tasks
        .map_err(|e| ApiError::BackendError(e.to_string()))?
        .into_iter()
        .map(|task| {
            task.try_map_args(|compact| {
                codec
                    .decode(&compact)
                    .map_err(|e| ApiError::CodecError(e.to_string()))
            })
        })
        .collect()
}

/// Get workers for a specific queue.
pub async fn get_workers<S>(storage: Arc<RwLock<S>>) -> Result<Vec<RunningWorker>, ApiError>
where
    S: ListWorkers,
    S::Error: std::error::Error,
{
    let storage = storage.read().await;
    storage
        .list_workers()
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))
}

/// Get a task by its ID.
pub async fn get_task_by_id<B, Args>(
    task_id: String,
    storage: Arc<RwLock<B>>,
) -> Result<Option<Task<Args>>, ApiError>
where
    B: FetchById + WireFormatBackend + 'static,
    B::Error: std::error::Error,
    B: Backend<Task = Task<B::Compact>>,
    B::Codec: Codec<Args, Compact = B::Compact>,
    <B::Codec as Codec<Args>>::Error: std::error::Error,
{
    let task_id = TaskId::from_str(&task_id).map_err(|e| ApiError::BackendError(e.to_string()))?;

    let mut storage = storage.write().await;
    let task = storage
        .fetch_by_id(&task_id)
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))?;
    let codec = storage.codec();
    task.map(|task| {
        task.try_map_args(|compact| {
            codec
                .decode(&compact)
                .map_err(|e| ApiError::CodecError(e.to_string()))
        })
    })
    .transpose()
}

/// Get all tasks across all queues.
pub async fn get_all_tasks<S>(
    storage: Arc<RwLock<S>>,
    filter: Filter,
) -> Result<Vec<Task<S::Compact>>, ApiError>
where
    S: ListAllTasks + Send,
    S::Compact: Serialize,
{
    storage
        .read()
        .await
        .list_all_tasks(&filter)
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))
}

/// Get all workers across all queues.
pub async fn get_all_workers<S>(storage: Arc<RwLock<S>>) -> Result<Vec<RunningWorker>, ApiError>
where
    S: ListWorkers,
    S::Error: std::error::Error,
{
    storage
        .read()
        .await
        .list_all_workers()
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))
}

/// Fetch all queues.
pub async fn fetch_queues<S>(storage: Arc<RwLock<S>>) -> Result<Vec<QueueInfo>, ApiError>
where
    S::Error: std::error::Error,
    S: ListQueues,
{
    storage
        .read()
        .await
        .list_queues()
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))
}

/// Get an overview of statistics.
pub async fn overview<S>(storage: Arc<RwLock<S>>) -> Result<Vec<Statistic>, ApiError>
where
    S::Error: std::error::Error,
    S: Metrics,
{
    let overview = storage.read().await.global().await;
    match overview {
        Ok(overview) => Ok(overview),
        Err(e) => Err(ApiError::BackendError(e.to_string())),
    }
}

/// A raw task structure that holds the task arguments and execution context.
#[derive(Debug, Clone)]
pub struct RawTask<Args>(pub Task<Args>);

impl<Args> Serialize for RawTask<Args>
where
    Args: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Task", 2)?;
        state.serialize_field("args", &self.0.args)?;
        state.serialize_field("ctx", self.0.ctx().as_ref())?;
        state.end()
    }
}

impl<'de, Args> Deserialize<'de> for RawTask<Args>
where
    Args: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TaskData<Args> {
            args: Args,
            ctx: ExecutionContext,
        }

        let data = TaskData::deserialize(deserializer)?;

        Ok(Self(Task::new(data.args).map_context(|_| data.ctx.into())))
    }
}
