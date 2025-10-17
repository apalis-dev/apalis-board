use clap::Parser;
/// Configuration for the Actix ntfy service
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub database_url: String,
    #[arg(short, long, default_value = "Notifications")]
    pub queue: String,
    #[arg(long, default_value = "localhost:8000")]
    pub host: String,
    #[arg(long, default_value = "https://ntfy.sh/")]
    pub ntfy_url: String,
    #[arg(long, default_value = "debug")]
    pub log_level: String,
    #[arg(long, default_value_t = 5)]
    pub worker_concurrency: usize,
    #[arg(long, default_value_t = 3)]
    pub retries: usize,
}
