# axum-email-service

Basic example that shows how to send emails via smtp using `lettre` and `axum`

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
    "index": 122,
    "text": "Welcome to our platform! Please verify your email.",
    "subject": "Welcome Aboard!",
    "to": "notify@mysite.com"
  }'
```

By default all emails are sent to a (dummy inbox)[https://www.wpoven.com/tools/free-smtp-server-for-testing]
