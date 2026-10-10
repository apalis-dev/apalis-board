use actix_web::Scope;
use actix_web::{rt::signal::ctrl_c, web, App, HttpServer};
use apalis::layers::retry::RetryPolicy;
use apalis::prelude::*;
use apalis_board::actix::framework::{ApiBuilder, RegisterRoute};
use apalis_board::actix::sse::TracingBroadcaster;
use apalis_board::actix::sse::TracingSubscriber;
use apalis_board::actix::ui::ServeUI;
use apalis_sqlite::shared::SqliteStorageFactory;
use apalis_sqlite::{Config, SqlitePool, SqliteStorage};
use clap::Parser;
use futures::{future, TryFutureExt};
use reqwest::Client;
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

    let pool = SqlitePool::connect(&args.database_url).await?;

    SqliteStorage::setup(&pool).await.unwrap();

    let config = Config::default().queue(&args.queue);

    let mut store = SqliteStorageFactory::new(&args.database_url);

    let notification_store = store.create_with_config(config).unwrap();

    let worker = WorkerBuilder::new("ntfy-banana")
        .backend(notification_store.clone())
        .retry(RetryPolicy::retries(args.retries))
        .concurrency(args.worker_concurrency)
        .enable_tracing()
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
                        .register(&args.queue, notification_store.clone())
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
