use apalis::prelude::{AbortError, BoxDynError, Data};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::cli::Args;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Notification {
    pub topic: String,
    pub body: String,
    pub title: String,
    pub priority: String,
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationRecord {
    pub id: String,
    pub time: i64,
    pub expires: i64,
    pub event: String,
}

pub async fn send_notification(
    task: Notification,
    client: Data<Client>,
    args: Data<Args>,
) -> Result<NotificationRecord, BoxDynError> {
    let url = format!("{}{}", args.ntfy_url, task.topic);
    let resp = client
        .post(url)
        .header("Title", task.title)
        .header("Priority", task.priority)
        .header("Tags", task.tags.join(","))
        .body(task.body)
        .send()
        .await?;
    let resp_text = resp
        .json::<NotificationRecord>()
        .await
        .map_err(AbortError::new)?; // Abort if response is not JSON
    Ok(resp_text)
}
