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

Here are the basics of setting up the board:

```rust
App::new()
    .app_data(web::Data::new(broadcaster)) // Pass in the broadcaster for realtime logs
    .service(
        ApiBuilder::new(Scope::new("/api/v1")) // Setup the mount
            .register(notification_store) // Add backends
            .register(email_store)
            .build(), // Build the routes
    )
    .service(ServeApp::new()) // Serve the frontend
```

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

- [bull-board](https://github.com/felixmosh/bull-board/)
