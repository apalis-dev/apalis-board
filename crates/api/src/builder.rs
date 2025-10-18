use apalis_core::backend::{Backend, FetchById, ListTasks, TaskSink, codec::Codec};
use serde::{Serialize, de::DeserializeOwned};

pub trait Builder<B, Compact> {
    type Output;

    fn register<T>(self, queue: &str) -> Self
    where
        T: Serialize + DeserializeOwned + 'static,
        B: ListTasks<T, Args = Compact> + FetchById<T>,
        B::Codec: Codec<T, Compact = Compact>,
        B: TaskSink<T>,
        <<B as Backend>::Codec as Codec<T>>::Error: std::error::Error;

    fn build(self) -> Self::Output;
}
