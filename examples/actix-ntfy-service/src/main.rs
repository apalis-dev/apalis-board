use actix_web::Scope;
use actix_web::{rt::signal::ctrl_c, web, App, HttpServer};
use apalis::layers::retry::RetryPolicy;
use apalis::prelude::*;
use apalis_board_api::framework::{ApiBuilder, RegisterRoute, ServeApp};
use apalis_board_api::logger::Subscriber;
use apalis_board_api::sse::Broadcaster;
use apalis_sqlite::{SqlitePool, SqliteStorage};
use clap::Parser;
use futures::{future, TryFutureExt};
use reqwest::Client;
use std::time::Duration;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

use crate::cli::Args;
use crate::notification::send_notification;

mod cli;
mod notification;

#[actix_web::main]
async fn main() -> Result<(), BoxDynError> {
    let args = Args::parse();
    let broadcaster = Broadcaster::create();

    let line_sub = Subscriber::new(&broadcaster);
    let tracer = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(EnvFilter::builder().parse(&args.log_level).unwrap()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .fmt_fields(tracing_subscriber::fmt::format::JsonFields::new())
                .event_format(tracing_subscriber::fmt::format().with_ansi(false).json())
                .with_writer(line_sub)
                .with_filter(EnvFilter::builder().parse(&args.log_level).unwrap()),
        );
    tracer.try_init().unwrap();
    let pool = SqlitePool::connect(&args.database_url).await.unwrap();
    SqliteStorage::setup(&pool).await.unwrap();

    let config = apalis_sqlite::Config::new(&args.queue).with_poll_interval(
        StrategyBuilder::new()
            .apply(
                IntervalStrategy::new(Duration::from_millis(100))
                    .with_backoff(BackoffConfig::default()),
            )
            .build(),
    );

    let notification_store = SqliteStorage::new_with_callback(&pool, &config);

    let worker = WorkerBuilder::new("ntfy-banana")
        .backend(notification_store.clone())
        .enable_tracing()
        .retry(RetryPolicy::retries(args.retries))
        .concurrency(args.worker_concurrency)
        .data(Client::new())
        .data(args.clone())
        .build(send_notification)
        .run_until(ctrl_c())
        .map_err(std::io::Error::other);

    let http = async move {
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(broadcaster.clone()))
                .service(
                    ApiBuilder::new(Scope::new("/api/v1"))
                        .register(notification_store.clone())
                        .build(),
                )
                .service(ServeApp::new())
        })
        .bind(&args.host)?
        .run()
        .await?;
        Ok(())
    };

    future::try_join(http, worker).await?;

    Ok(())
}
