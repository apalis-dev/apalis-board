# <img alt="apalis-board" src="https://github.com/apalis-dev/apalis-board/raw/master/screenshots/logo.svg" width="24px" /> apalis-board

Apalis board contains a number of crates useful for building UIs and apis for [apalis](https://github.com/geofmureithi/apalis) backends.

**Key features:**

- Visualize your queues and jobs in real time
- Beautiful UI to track job status and progress
- Perform actions on jobs directly from the dashboard
- Gain insights into queue health and worker activity
- Easily integrate with existing apalis-based services
- Streamline job management and debugging

Get a clear overview of what's happening in your queues and manage jobs efficiently.

## Crates

- `apalis-board-types`: Default types used around
- `apalis-board-api`: Provides api utilities for `axum` and `actix`
- `apalis-board`: Provides the UI interface written in `leptos`

## Usage

Each version of `apalis-board-api` includes a compatible version of the ui so you only need to include one dep.

```toml
apalis-board-api = { version = "1.0.0-alpha.2", features = ["actix"] } #Or axum
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

## Screenshots

### Tasks

![Tasks](https://github.com/apalis-dev/apalis-board/raw/master/screenshots/tasks.png)

### Single Task

![Tasks](https://github.com/apalis-dev/apalis-board/raw/master/screenshots/task.png)

### Workers

![Workers](https://github.com/apalis-dev/apalis-board/raw/master/screenshots/workers.png)

### Queues

![Queues](https://github.com/apalis-dev/apalis-board/raw/master/screenshots/queues.png)

## Building the frontend

```sh
cd crates/board
trunk build
```

## Examples

- axum-email-service : Basic example that shows how to send emails via smtp using `lettre` and `axum`
- actix-ntfy-service : Basic example that shows how to publish notifications using `ntfy.sh` and `actix`

## Acknowledgments

The following repos were referenced in building the frontend

- [bull-board](https://github.com/felixmosh/bull-board/)
- [trigger.dev](https://github.com/triggerdotdev/trigger.dev)
