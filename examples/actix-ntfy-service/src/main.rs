use actix_web::Scope;
use actix_web::{rt::signal::ctrl_c, web, App, HttpServer};
use apalis::layers::retry::RetryPolicy;
use apalis::prelude::*;
use apalis_board_api::framework::{ApiBuilder, RegisterRoute};
use apalis_board_api::sse::TracingBroadcaster;
use apalis_board_api::sse::TracingSubscriber;
use apalis_board_api::ui::ServeUI;
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

    let broadcaster = TracingBroadcaster::create();

    let tracing_subscriber = TracingSubscriber::new(&broadcaster);
    let tracing_layer = tracing_subscriber
        .layer()
        .with_filter(EnvFilter::builder().parse(&args.log_level).unwrap());

    let stdio_layer = tracing_subscriber::fmt::layer()
        .with_filter(EnvFilter::builder().parse(&args.log_level).unwrap());

    tracing_subscriber::registry()
        .with(stdio_layer)
        .with(tracing_layer)
        .init();
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
                .app_data(web::Data::new(broadcaster.clone())) // Add the broadcaster to the app data
                .service(
                    ApiBuilder::new(Scope::new("/api/v1"))
                        .register(notification_store.clone())
                        .build(),
                )
                .service(ServeUI::new())
        })
        .bind(&args.host)?
        .run()
        .await?;
        Ok(())
    };

    future::try_join(http, worker).await?;

    Ok(())
}
