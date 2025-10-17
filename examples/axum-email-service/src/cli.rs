use clap::Parser;
/// Configuration for the Actix ntfy service
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub database_url: String,
    #[arg(short, long, default_value = "Emails")]
    pub queue: String,
    #[arg(long, default_value = "localhost:8000")]
    pub api_host: String,
    #[arg(long, default_value = "smtp.freesmtpservers.com")]
    pub smtp_host: String,
    #[arg(long, default_value_t = 25)]
    pub smtp_port: u16,
    #[arg(long)]
    pub smtp_user: Option<String>,
    #[arg(long)]
    pub smtp_password: Option<String>,
    #[arg(long, default_value = "trace")] // To get lettre debug logs
    pub log_level: String,
    #[arg(long, default_value_t = 5)]
    pub worker_concurrency: usize,
    #[arg(long, default_value_t = 3)]
    pub retries: usize,
}
