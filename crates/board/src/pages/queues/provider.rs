use leptos_struct_table::{ColumnSort, PaginatedTableDataProvider};
use std::collections::VecDeque;

use crate::{api::ApiClient, pages::queues::Queue};

pub struct QueueProvider {}

impl Default for QueueProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl PaginatedTableDataProvider<Queue> for QueueProvider {
    const PAGE_ROW_COUNT: usize = 100;

    async fn get_page(&self, page_index: usize) -> Result<Vec<Queue>, String> {
        let resp: Vec<Queue> = ApiClient::get("/").await.map_err(|e| e.to_string())?;

        Ok(resp)
    }

    async fn row_count(&self) -> Option<usize> {
        Some(1)
    }
}
