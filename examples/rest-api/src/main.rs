use actix::clock::sleep;
use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{rt::signal::ctrl_c, web, App, HttpServer};
use apalis::prelude::*;
use apalis_board_utils::sse::Broadcaster;
use apalis_redis::RedisStorage;
use api::ApiBuilder;
use logger::Subscriber;
use std::fmt::Debug;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer};

use futures::future;
use log::info;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use tracing_subscriber::util::SubscriberInitExt;
mod api;
mod logger;
mod sse {
    use std::sync::Arc;

    use actix_web::{
        http::header::{ContentEncoding, ContentLength, CONNECTION, CONTENT_TYPE},
        web::*,
        HttpResponse,
    };
    use apalis_board_utils::sse::Broadcaster;
    use std::sync::Mutex;

    pub async fn new_client(broadcaster: Data<Arc<Mutex<Broadcaster>>>) -> HttpResponse {
        let rx = broadcaster.lock().unwrap().new_client();
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/event-stream"))
            .insert_header(("cache-control", "no-cache"))
            .insert_header((CONNECTION, "keep-alive"))
            .insert_header(ContentLength(0))
            .insert_header(ContentEncoding::Identity)
            .streaming(rx)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub text: String,
}

pub async fn send_email(job: Email) -> String {
    log::info!("Attempting to send email to {}", job.to);
    sleep(Duration::from_secs(5)).await;
    log::info!("Checking DNS");
    sleep(Duration::from_secs(1)).await;
    log::info!("Checking MX");
    sleep(Duration::from_secs(1)).await;
    log::info!("Checking DB");
    sleep(Duration::from_secs(1)).await;
    log::info!("Creating connection");
    sleep(Duration::from_secs(1)).await;
    log::info!("Success");
    job.to
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,sqlx::query=error");
    let broadcaster = Broadcaster::create();

    let line_sub = Subscriber {
        tx: broadcaster.clone(),
    };
    let tracer = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(EnvFilter::builder().parse("debug").unwrap()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .fmt_fields(tracing_subscriber::fmt::format::JsonFields::new())
                .event_format(tracing_subscriber::fmt::format().with_ansi(false).json())
                .with_writer(line_sub)
                .with_filter(EnvFilter::builder().parse("debug").unwrap()),
        );
    tracer.try_init().unwrap();

    let mut redis = RedisStorage::new(apalis_redis::connect("redis://127.0.0.1/").await.unwrap());

    let b = broadcaster.clone();

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;
            b.lock().unwrap().remove_stale_clients();
        }
    });

    produce_redis_jobs(&mut redis).await;
    let worker = Monitor::new()
        .register(
            WorkerBuilder::new("tasty-apple")
                .enable_tracing()
                .backend(redis.clone())
                .build_fn(send_email),
        )
        .on_event(|e| info!("{e}"))
        .run_with_signal(async { ctrl_c().await });
    let http = async move {
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(broadcaster.clone()))
                .route("/api/v1/events", web::get().to(sse::new_client))
                .service(
                    web::scope("/api").service(
                        ApiBuilder::new()
                            .add_storage(&redis, &redis.get_config().get_namespace())
                            .build(),
                    ),
                )
                .service(
                    Files::new("/", "./crates/board/dist/")
                        .prefer_utf8(true)
                        .index_file("index.html")
                        .default_handler(fn_service(|req: ServiceRequest| async {
                            let (req, _) = req.into_parts();
                            let file =
                                NamedFile::open_async("./crates/board/dist/index.html").await?;
                            let res = file.into_response(&req);
                            Ok(ServiceResponse::new(req, res))
                        })),
                )
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await?;
        Ok(())
    };

    future::try_join(http, worker).await?;

    Ok(())
}

async fn produce_redis_jobs(storage: &mut RedisStorage<Email>) {
    use apalis::prelude::Storage;
    for i in 0..10 {
        storage
            .push(Email {
                to: format!("test{i}@example.com"),
                text: "Test background job from apalis".to_string(),
                subject: "Background email job".to_string(),
            })
            .await
            .unwrap();
    }
}
