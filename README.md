# <img alt="apalis-board" src="screenshots/logo.svg" width="24px" /> apalis-board

[![CI](https://github.com/apalis-dev/apalis-board/workflows/All%20Checks/badge.svg)](https://github.com/apalis-dev/apalis-board/actions/workflows/ci.yml)
[![Rust CI](https://github.com/apalis-dev/apalis-board/workflows/Rust%20CI/badge.svg)](https://github.com/apalis-dev/apalis-board/actions/workflows/rust-ci.yml)
[![Frontend Build](https://github.com/apalis-dev/apalis-board/workflows/Frontend%20Build/badge.svg)](https://github.com/apalis-dev/apalis-board/actions/workflows/frontend-build.yml)
[![Security Audit](https://github.com/apalis-dev/apalis-board/workflows/Security%20Audit/badge.svg)](https://github.com/apalis-dev/apalis-board/actions/workflows/security.yml)
[![codecov](https://codecov.io/gh/apalis-dev/apalis-board/branch/master/graph/badge.svg)](https://codecov.io/gh/apalis-dev/apalis-board)

Apalis board contains a number of crates useful for building UIs and apis for [apalis](https://github.com/geofmureithi/apalis) backends.


**Key features:**
- Visualize your queues and jobs in real time
- Beautiful UI to track job status and progress
- Perform actions on jobs directly from the dashboard
- Gain insights into queue health and worker activity
- Easily integrate with existing apalis-based services
- Streamline job management and debugging

Get a clear overview of what's happening in your queues and manage jobs efficiently.

## Development

### Prerequisites

- Rust (latest stable)
- Node.js 20+
- Trunk: `cargo install trunk`
- wasm32 target: `rustup target add wasm32-unknown-unknown`

### Building

```sh
# Install Tailwind CSS dependencies
npm ci

# Build the frontend
cd crates/board
trunk build

# Build the backend crates
cd ../..
cargo build --workspace
```

For detailed development instructions, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Screenshots

### Tasks

![Tasks](screenshots/tasks.png)

### Single Task

![Tasks](screenshots/task.png)

### Workers

![Workers](screenshots/workers.png)

### Queues

![Queues](screenshots/queues.png)

## Building the frontend

```sh
cd crates/board
trunk build
```

## Examples

- **axum-email-service**: Basic example showing how to send emails via SMTP using `lettre` and `axum`
- **actix-ntfy-service**: Basic example showing how to publish notifications using `ntfy.sh` and `actix`

Note: Examples depend on external repositories and are not part of the main workspace build.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Backlog

- [ ] Complete the TaskPage
- [ ] Improve the Logs Page
- [ ] Some more cleanup?

## Acknowledgments
- https://github.com/felixmosh/bull-board
