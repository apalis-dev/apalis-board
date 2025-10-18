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

- axum-email-service : Basic example that shows how to send emails via smtp using `lettre` and `axum`
- actix-ntfy-service : Basic example that shows how to publish notifications using `ntfy.sh` and `actix`

## Backlog

- [ ] Complete the TaskPage
- [ ] Improve the Logs Page
- [ ] Some more cleanup?

## Acknowledgments
- https://github.com/felixmosh/bull-board
