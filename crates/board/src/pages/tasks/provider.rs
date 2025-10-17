use apalis_core::{backend::Statistic, task::status::Status};
use leptos_struct_table::{ColumnSort, PaginatedTableDataProvider};
use std::collections::VecDeque;

use crate::{api::ApiClient, pages::tasks::Task, RawTask};

pub struct TaskProvider {
    sorting: VecDeque<(usize, ColumnSort)>,
    queue: Option<String>,
    status: Option<Status>,
}

impl TaskProvider {
    pub fn all(status: Status) -> Self {
        Self {
            sorting: VecDeque::new(),
            queue: None,
            status: Some(status),
        }
    }
    pub fn new(queue: String) -> Self {
        Self {
            sorting: VecDeque::new(),
            queue: Some(queue),
            status: None,
        }
    }

    pub fn new_with_status(queue: String, status: Status) -> Self {
        Self {
            sorting: VecDeque::new(),
            queue: Some(queue),
            status: Some(status),
        }
    }
    fn url_sort_param_for_column(&self, column: usize) -> &'static str {
        match column {
            0 => "task_id",
            1 => "attempt",
            2 => "status",
            3 => "run_at",
            4 => "ctx",
            _ => "",
        }
    }

    fn url_sort_param_for_sort_pair(&self, pair: &(usize, ColumnSort)) -> String {
        let col = self.url_sort_param_for_column(pair.0);

        let dir = match pair.1 {
            ColumnSort::Ascending => "asc",
            ColumnSort::Descending => "desc",
            ColumnSort::None => return "".to_string(),
        };

        format!("sort={col}:{dir}")
    }

    fn get_url(&self, page_index: usize) -> String {
        let mut sort = String::new();
        for pair in &self.sorting {
            sort.push_str(&self.url_sort_param_for_sort_pair(pair));
        }
        let prefix = match &self.queue {
            None => "".to_string(),
            Some(queue) => format!("/queues/{queue}"),
        };
        format!(
            "{prefix}/tasks?{sort}&page={}&page_size={}&queue={}&status={}",
            page_index + 1,
            Self::PAGE_ROW_COUNT,
            self.queue.as_deref().unwrap_or(""),
            self.status
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or("".to_string()),
        )
    }
}

impl PaginatedTableDataProvider<Task> for TaskProvider {
    const PAGE_ROW_COUNT: usize = 15;

    async fn get_page(&self, page_index: usize) -> Result<Vec<Task>, String> {
        let url = self.get_url(page_index);
        let resp: Vec<RawTask> = ApiClient::get(&url).await?;
        let tasks = resp
            .into_iter()
            .map(|t| Task {
                args: t.args,
                task_id: t.parts.task_id.unwrap().to_string(),
                attempt: t.parts.attempt.current(),
                status: t.parts.status.load(),
                run_at: t.parts.run_at,
                meta: t.parts.ctx,
            })
            .collect();
        Ok(tasks)
    }

    async fn row_count(&self) -> Option<usize> {
        let queue = self.queue.as_deref().unwrap_or("");
        let url = format!("/queues/{queue}/stats");
        let resp: Vec<Statistic> = ApiClient::get(&url).await.ok()?;
        let total = resp.iter().find(|s| s.title == "Pending")?;
        total.value.parse().ok()
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sorting = sorting.clone();
    }
}
