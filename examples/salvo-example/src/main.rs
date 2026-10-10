//! # salvo-example
//!
//! A complete example that wires **apalis-board** into a [Salvo](https://salvo.rs/)
//! server.
//!
//! ## Running
//!
//! ```shell
//! cargo run --package salvo-example
//! # Open: http://localhost:5800
//! ```

use apalis::prelude::*;
use apalis_board::salvo::framework::salvo::ui::ServeApp;
use apalis_board::salvo::{
    framework::{ApiBuilder, RegisterRoute},
    sse::{TracingBroadcaster, TracingSubscriber},
};
use apalis_sqlite::{SqlitePool, SqliteStorage};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::info;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Email {
    to: String,
    subject: String,
}

async fn send_email(email: Email) {
    info!(
        to = %email.to,
        subject = %email.subject,
        "📧 processing email job"
    );

    tokio::time::sleep(Duration::from_millis(150)).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let broadcaster = TracingBroadcaster::create();
    let tracing_subscriber_inner = TracingSubscriber::new(&broadcaster);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber_inner
                .layer()
                .with_filter(EnvFilter::builder().parse("debug").unwrap()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = SqlitePool::connect("sqlite:example.db?mode=rwc").await?;
    SqliteStorage::setup(&pool).await?;

    let email_storage = SqliteStorage::new(&pool);

    let mut producer = email_storage.clone();
    tokio::spawn(async move {
        let mut n = 0u32;
        loop {
            n += 1;
            let job = Email {
                to: format!("user{n}@example.com"),
                subject: format!("Newsletter #{n}"),
            };
            if let Err(e) = producer.push(job).await {
                tracing::error!("push failed: {e}");
            }
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    let api_router = ApiBuilder::new(Router::new())
        .register(std::any::type_name::<Email>(), email_storage.clone())
        .build();

    let app = Router::new()
        .hoop(affix_state::inject(broadcaster))
        .push(Router::with_path("api/v1").push(api_router))
        .push(ServeApp::router());

    let monitor = Monitor::new().register(move |index| {
        WorkerBuilder::new(format!("email-worker-{index}"))
            .backend(email_storage.clone())
            .enable_tracing()
            .build(send_email)
    });

    info!("🚀 apalis-board (Salvo) → http://0.0.0.0:5800");

    tokio::select! {
        result = async {
            let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
            Server::new(acceptor).serve(app).await;
            Ok::<_, anyhow::Error>(())
        } => result?,

        result = monitor.run() => result?,
    }

    Ok(())
}
