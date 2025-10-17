use crate::pages::workers::Worker;
use gloo_net::http::Request;
use leptos::prelude::{GetUntracked, Signal, Track};
use leptos_struct_table::{ColumnSort, TableDataProvider};
use std::{collections::VecDeque, ops::Range};

pub struct WorkerProvider {
    sorting: VecDeque<(usize, ColumnSort)>,
    queue: Option<Signal<String>>,
}

impl WorkerProvider {
    pub fn all() -> Self {
        Self {
            sorting: VecDeque::new(),
            queue: None,
        }
    }
    pub fn new(queue: Signal<String>) -> Self {
        Self {
            sorting: VecDeque::new(),
            queue: Some(queue),
        }
    }
    fn url_sort_param_for_column(&self, column: usize) -> &'static str {
        match column {
            0 => "name",
            1 => "backend",
            2 => "started_at",
            3 => "last_heartbeat",
            4 => "service",
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

    fn get_url(&self) -> String {
        const API_PATH: &str = "/api/v1";
        let mut sort = String::new();
        for pair in &self.sorting {
            sort.push_str(&self.url_sort_param_for_sort_pair(pair));
        }
        match self.queue.map(|s| s.get_untracked()) {
            Some(ref q) if !q.is_empty() => {
                format!("{API_PATH}/queues/{q}/workers?{sort}",)
            }
            _ => {
                format!("{API_PATH}/workers?{sort}",)
            }
        }
    }
}

impl TableDataProvider<Worker> for WorkerProvider {
    async fn get_rows(&self, range: Range<usize>) -> Result<(Vec<Worker>, Range<usize>), String> {
        let url = self.get_url();
        let Range { start, end } = range;

        let resp: Vec<Worker> = Request::get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;
        if start > 0 {
            return Ok((vec![], start..start));
        }
        let r = start..start + resp.len();

        Ok((resp, r))
    }

    async fn row_count(&self) -> Option<usize> {
        let url = self.get_url();
        let resp: Vec<Worker> = Request::get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())
            .ok()?
            .json()
            .await
            .map_err(|e| e.to_string())
            .ok()?;
        Some(resp.len())
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sorting = sorting.clone();
    }

    fn track(&self) {
        if let Some(q) = self.queue {
            q.track()
        }
    }
}
