#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
use std::{str::FromStr, sync::Arc};

use apalis_board_types::ApiError;
use apalis_core::{
    backend::{
        Backend, BackendExt, FetchById, Filter, ListAllTasks, ListQueues, ListTasks, ListWorkers,
        Metrics, QueueInfo, RunningWorker, Statistic, TaskSink, codec::Codec,
    },
    task::{Task, builder::TaskBuilder, task_id::TaskId},
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::RwLock;

pub mod framework;

#[cfg(feature = "sse")]
pub mod sse;
#[cfg(feature = "ui")]
pub mod ui;

pub async fn push_task<Args, B, Compact>(
    _queue: String,
    task: Args,
    storage: Arc<RwLock<B>>,
) -> Result<(), ApiError>
where
    Args: Serialize + DeserializeOwned + 'static,
    B: TaskSink<Args> + Send + BackendExt,
    B::Error: std::error::Error,
    B::Codec: Codec<Args, Compact = Compact>,
    <<B as BackendExt>::Codec as Codec<Args>>::Error: std::error::Error,
{
    let task = TaskBuilder::new(task).build();
    let res = storage.write().await.push_task(task).await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(ApiError::BackendError(e.to_string())),
    }
}

pub async fn stats_by_queue<S>(
    storage: Arc<RwLock<S>>,
    queue: String,
) -> Result<Vec<Statistic>, ApiError>
where
    S::Error: std::error::Error,
    S: Metrics,
{
    let stats = storage.read().await.fetch_by_queue(queue.as_ref()).await;
    match stats {
        Ok(stats) => Ok(stats),
        Err(e) => Err(ApiError::BackendError(e.to_string())),
    }
}

pub async fn get_tasks<S, T, Compact>(
    queue: String,
    storage: Arc<RwLock<S>>,
    filter: Filter,
) -> Result<Vec<Task<T, S::Context, S::IdType>>, ApiError>
where
    T: Serialize + DeserializeOwned + 'static,
    S: ListTasks<T> + Send + BackendExt,
    S::Context: Serialize,
    S::IdType: Serialize,
    <S as Backend>::Error: std::error::Error,
    S::Codec: Codec<T, Compact = Compact>,
{
    storage
        .read()
        .await
        .list_tasks(queue.as_ref(), &filter)
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))
}

pub async fn get_workers<S>(
    storage: Arc<RwLock<S>>,
    queue: String,
) -> Result<Vec<RunningWorker>, ApiError>
where
    S: ListWorkers,
    S::Error: std::error::Error,
{
    storage
        .read()
        .await
        .list_workers(queue.as_ref())
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))
}

pub async fn get_task_by_id<B, T>(
    task_id: String,
    storage: Arc<RwLock<B>>,
) -> Result<Option<Task<T, B::Context, B::IdType>>, ApiError>
where
    T: Serialize + DeserializeOwned + 'static,
    B: FetchById<T> + 'static,
    B::Context: Serialize,
    B::IdType: Serialize,
    B::Context: Serialize,
    B::Error: std::error::Error,
    B::IdType: FromStr,
    <<B as Backend>::IdType as FromStr>::Err: std::error::Error,
{
    let task_id = TaskId::<B::IdType>::from_str(&task_id)
        .map_err(|e| ApiError::BackendError(e.to_string()))?;

    storage
        .write()
        .await
        .fetch_by_id(&task_id)
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))
}

pub async fn get_all_tasks<S>(
    storage: Arc<RwLock<S>>,
    filter: Filter,
) -> Result<Vec<Task<S::Compact, S::Context, S::IdType>>, ApiError>
where
    S: ListAllTasks + Send,
    S::Context: Serialize,
    S::IdType: Serialize,
    S::Compact: Serialize,
    <S as Backend>::Error: std::error::Error,
    <<S as BackendExt>::Codec as Codec<<S as Backend>::Args>>::Error: std::error::Error,
{
    storage
        .read()
        .await
        .list_all_tasks(&filter)
        .await
        .map_err(|e| ApiError::BackendError(e.to_string()))
}

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
