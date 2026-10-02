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

// ─── Job type ─────────────────────────────────────────────────────────────────

/// A simple "send email" background job.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Email {
    to: String,
    subject: String,
}

/// Handler — would actually send an email here.
async fn send_email(email: Email) {
    info!(
        to = %email.to,
        subject = %email.subject,
        "📧 processing email job"
    );
    // Simulate I/O work.
    tokio::time::sleep(Duration::from_millis(150)).await;
}

// ─── Main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ── 1. TracingBroadcaster ─────────────────────────────────────────────────
    //
    // This broadcasts every tracing span/event emitted by apalis workers to all
    // SSE subscribers connected at GET /api/v1/events.

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

    // ── 2. SQLite storage ─────────────────────────────────────────────────────
    let pool = SqlitePool::connect("sqlite:example.db?mode=rwc").await?;
    SqliteStorage::setup(&pool).await?;

    let email_storage = SqliteStorage::new(&pool);

    // ── 3. Background job producer ────────────────────────────────────────────
    let mut producer = email_storage.clone();
    tokio::spawn(async move {
        let mut n = 0u32;
        loop {
            n += 1;
            let job = Email {
                to: format!("user{}@example.com", n),
                subject: format!("Newsletter #{}", n),
            };
            if let Err(e) = producer.push(job).await {
                tracing::error!("push failed: {e}");
            }
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    // ── 5. apalis-board API router (Salvo) ────────────────────────────────────
    let api_router = ApiBuilder::new(Router::new())
        .register(email_storage.clone())
        .build();

    // ── 6. Full Salvo router ──────────────────────────────────────────────────
    //
    // Route layout:
    //   /api/v1/**   → board REST API
    //   /**          → embedded board UI (SPA)
    let app = Router::new()
        .hoop(affix_state::inject(broadcaster))
        .push(Router::with_path("api/v1").push(api_router))
        .push(ServeApp::router());

    // ── 7. Run both the Salvo server and the apalis monitor concurrently ───────
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
