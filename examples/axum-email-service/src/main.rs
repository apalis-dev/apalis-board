use std::time::Duration;

use apalis::{
    layers::{WorkerBuilderExt, retry::RetryPolicy},
    prelude::{
        BackoffConfig, BoxDynError, Codec, Data, IntervalStrategy, StrategyBuilder, WorkerBuilder,
    },
};
use apalis_board::axum::{
    framework::{ApiBuilder, RegisterRoute},
    sse::{TracingBroadcaster, TracingSubscriber},
    ui::ServeUI,
};
use apalis_postgres::PostgresStorage;
use axum::{Extension, Router, ServiceExt};
use clap::Parser;
use futures::{FutureExt, TryFutureExt};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Mechanism,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sqlx::PgPool;
use tokio::signal::ctrl_c;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use tracing_subscriber::{
    EnvFilter, Layer as TraceLayer, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::cli::Args;

mod cli;

pub type MailClient = AsyncSmtpTransport<Tokio1Executor>;

#[derive(Clone)]
struct MessagePack;

impl<T: Serialize + DeserializeOwned> Codec<T> for MessagePack {
    type Compact = Vec<u8>;
    type Error = MessagePackError;
    fn encode(input: &T) -> Result<Vec<u8>, Self::Error> {
        rmp_serde::to_vec(input).map_err(|e| e.into())
    }

    fn decode(compact: &Vec<u8>) -> Result<T, Self::Error> {
        rmp_serde::from_slice(compact).map_err(|e| e.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum MessagePackError {
    #[error("Encoding error: {0}")]
    Encode(#[from] rmp_serde::encode::Error),
    #[error("Decoding error: {0}")]
    Decode(#[from] rmp_serde::decode::Error),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub text: String,
    pub index: usize,
}

pub async fn send_email(task: Email, client: Data<MailClient>) -> Result<String, BoxDynError> {
    log::info!("Sending email to {}", task.to);
    let message = Message::builder()
        .from("John Smith <example@email.com>".parse()?)
        .to(task.to.parse()?)
        .subject(&task.subject)
        .body(task.text)?;
    log::debug!("Email message created: {:?}", message.headers());
    client.send(message).await?;
    log::warn!("Email sent to {}", task.to);

    log::trace!("Email Trace sent to {}", task.to);
    Ok(format!("Email sent to {}", task.to))
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let broadcaster = TracingBroadcaster::create();

    let client: MailClient =
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(args.smtp_host)
            .port(args.smtp_port)
            .authentication(vec![Mechanism::Plain])
            .build();

    let line_sub = TracingSubscriber::new(&broadcaster);
    let tracer = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(EnvFilter::builder().parse(&args.log_level).unwrap()),
        )
        .with(
            line_sub
                .layer()
                .with_filter(EnvFilter::builder().parse(&args.log_level).unwrap()),
        );
    tracer.try_init().unwrap();
    let pool = PgPool::connect(&args.database_url).await.unwrap();
    PostgresStorage::setup(&pool).await.unwrap();

    let config = apalis_sql::config::Config::new(&args.queue).with_poll_interval(
        StrategyBuilder::new()
            .apply(
                IntervalStrategy::new(Duration::from_secs(1))
                    .with_backoff(BackoffConfig::default()),
            )
            .build(),
    );
    let email_store = PostgresStorage::new_with_notify(&pool, &config).with_codec::<MessagePack>();

    let email_worker = WorkerBuilder::new("lettre-email-worker")
        .backend(email_store.clone())
        .retry(RetryPolicy::retries(args.retries))
        .enable_tracing()
        .concurrency(args.worker_concurrency)
        .data(client)
        .build(send_email)
        .run_until(ctrl_c())
        .map_err(std::io::Error::other);

    let http = async move {
        let api = ApiBuilder::new(Router::new())
            .register(email_store.clone())
            .build();
        let layer = NormalizePathLayer::trim_trailing_slash();
        let router = Router::new()
            .nest("/api/v1", api)
            .fallback_service(ServeUI::new())
            .layer(Extension(broadcaster.clone()));

        let listener = tokio::net::TcpListener::bind(&args.api_host).await.unwrap();
        let app = ServiceExt::<axum::extract::Request>::into_make_service(layer.layer(router));
        axum::serve(listener, app)
            .with_graceful_shutdown(ctrl_c().map(|_| ()))
            .await
    };

    let res = futures::future::try_select(email_worker.boxed(), http.boxed()).await;
    match res {
        Ok(futures::future::Either::Left((_res, _http))) => {
            log::info!("Email worker has exited");
        }
        Ok(futures::future::Either::Right((_res, _worker))) => {
            log::info!("HTTP server has exited");
            log::info!("Shutting down");
        }
        Err(futures::future::Either::Left((_res, _http))) => {
            log::error!("An error in worker occurred {_res}");
        }
        Err(futures::future::Either::Right((_res, _worker))) => {
            log::error!("An error occurred in http {_res}");
        }
    }
}
