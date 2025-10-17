# actix-ntfy-service

Basic example that shows how to publish notifications
The example uses `ntfy.sh` for simplicity and `actix` as a webserver

## Compiling

1. Compile the `frontend`:

```
cd board
trunk build --release
```

2. Run the `backend`

```
cargo run -- -d <path to sqlite db>
```

You can now access the dashboard at `localhost:8000`

### Push some notifications

```sh
curl -X PUT http://127.0.0.1:8000/api/v1/queues/Notifications/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "topic": "666",
    "body": "Welcome to our platform! Please verify your email.",
    "title": "Welcome Aboard!",
    "priority": "high",
    "tags": ["welcome"]
  }'
```

Verify by using any applicable `ntf.sh`
