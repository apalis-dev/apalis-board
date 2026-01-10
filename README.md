# apalis-board

<div align="center">
    <img alt="apalis-board" src="https://github.com/apalis-dev/apalis-board/raw/main/screenshots/logo.svg" width="180px" />
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/apalis-board">
    <img src="https://img.shields.io/crates/v/apalis-board.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/apalis-board">
    <img src="https://img.shields.io/crates/d/apalis-board.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/apalis-board">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <a href="https://github.com/apalis-dev/apalis/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/apalis-dev/apalis-board/ci.yml?branch=main&style=flat-square"
      alt="CI" />
  </a>
</div>
<br/>

`apalis-board` provides utilities for building web interfaces and apis for managing [apalis](https://github.com/apalis-dev/apalis) backends.

**Key features:**

- Visualize your queues and jobs in real time
- Beautiful UI to track job status and progress
- Perform actions on jobs directly from the dashboard
- Gain insights into queue health and worker activity
- Easily integrate with existing `apalis`-based services
- Streamline job management and debugging

Get a clear overview of what's happening in your queues and manage jobs efficiently.

## Crates

- [`apalis-board-types`](https://docs.rs/apalis-board-types): Default types used around
- [`apalis-board-api`](https://docs.rs/apalis-board-api): Provides api utilities for `axum` and `actix`
- [`apalis-board-web`](https://docs.rs/apalis-board-web): Provides the UI interface written in `leptos`

## Usage

Each version of `apalis-board` includes a compatible version of the ui.

```toml
apalis-board = { version = "1.0.0-rc.2", features = ["actix"] } #Or axum
```

Here are the basics of setting up the board:

```rust,ignore
App::new()
    .service(
        ApiBuilder::new(Scope::new("/api/v1")) // Setup the mount
            .register(notification_store) // Add backends
            .register(email_store)
            .build(), // Build the routes an
    )
    .service(ServeApp::new()) // Serve the frontend
```

### Including Realtime tracing events

```rust,ignore
let broadcaster = TracingBroadcaster::create();

let tracing_subscriber = TracingSubscriber::new(&broadcaster);
let tracing_layer = tracing_subscriber.layer()
    .with_filter(EnvFilter::builder().parse("debug").unwrap());


tracing_subscriber::registry().with(tracing_layer).init();

/// Then register the broadcaster
App::new()
    .app_data(broadcaster.clone())

```

If you visit `/api/v1/events` you will receive the task logs. This is also accessible on the `/logs` page in the board.

## Leptos integration

If you are working on a leptos UI and want to embed the web interface in part of in full, then you can import the `web` functionality:

```toml
apalis-board = { version = "1.0.0-rc.2", features = ["web"] }
```

## Support

| Source                | Crate                                                                                                                               | Support |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | ------- |
| `apalis-cron`         | <a href="https://docs.rs/apalis-cron"><img src="https://img.shields.io/crates/v/apalis-cron?style=flat-square"></a>                 | ❌      |
| `apalis-redis`        | <a href="https://docs.rs/apalis-redis"><img src="https://img.shields.io/crates/v/apalis-redis?style=flat-square"></a>               | ⚠️      |
| `apalis-sqlite`       | <a href="https://docs.rs/apalis-sqlite"><img src="https://img.shields.io/crates/v/apalis-sqlite?style=flat-square"></a>             | ✅      |
| `apalis-postgres`     | <a href="https://docs.rs/apalis-postgres"><img src="https://img.shields.io/crates/v/apalis-postgres?style=flat-square"></a>         | ✅      |
| `apalis-mysql`        | <a href="https://docs.rs/apalis-mysql"><img src="https://img.shields.io/crates/v/apalis-mysql?style=flat-square"></a>               | ✅      |
| `apalis-amqp`         | <a href="https://docs.rs/apalis-amqp"><img src="https://img.shields.io/crates/v/apalis-amqp?style=flat-square"></a>                 | ⌛⚠️    |
| `apalis-rsmq`         | <a href="https://docs.rs/apalis-rsmq"><img src="https://img.shields.io/crates/v/apalis-rsmq?style=flat-square"></a>                 | ⌛      |
| `apalis-pgmq`         | <a href="https://docs.rs/apalis-pgmq"><img src="https://img.shields.io/crates/v/apalis-pgmq?style=flat-square"></a>                 | ⌛      |
| `apalis-file-storage` | <a href="https://docs.rs/apalis-file-storage"><img src="https://img.shields.io/crates/v/apalis-file-storage?style=flat-square"></a> | ⌛⚠️    |

## Screenshots

### Tasks

![Tasks](https://github.com/apalis-dev/apalis-board/raw/main/screenshots/tasks.png)

### Single Task

![Tasks](https://github.com/apalis-dev/apalis-board/raw/main/screenshots/task.png)

### Workers

![Workers](https://github.com/apalis-dev/apalis-board/raw/main/screenshots/workers.png)

### Queues

![Queues](https://github.com/apalis-dev/apalis-board/raw/main/screenshots/queues.png)

## Building the frontend

```sh
cd crates/web
trunk build
```

## Examples

- [`axum-email-service`](https://github.com/apalis-dev/apalis-board/tree/main/examples/axum-email-service) : Basic example that shows how to send emails via smtp using `lettre` and `axum`
- [`actix-ntfy-service`](https://github.com/apalis-dev/apalis-board/tree/main/examples/actix-ntfy-service) : Basic example that shows how to publish notifications using `ntfy.sh` and `actix`

## Acknowledgments

The following repos were referenced in building the frontend

- [bull-board](https://github.com/felixmosh/bull-board/)
- [trigger.dev](https://github.com/triggerdotdev/trigger.dev)
