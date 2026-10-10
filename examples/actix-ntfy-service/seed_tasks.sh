#!/usr/bin/env bash
# Seeds N sample tasks with random future run times.
# Usage:  ./seed_tasks.sh            # send 1000 tasks
#         DRY_RUN=1 ./seed_tasks.sh  # just print the curl commands
#         COUNT=50 MAX_DAYS=1 ./seed_tasks.sh

URL="${URL:-http://127.0.0.1:8000/api/v1/queues/Notifications/tasks}"
COUNT="${COUNT:-1000}"
MIN_DELAY="${MIN_DELAY:-60}"                    # earliest: 1 minute from now
MAX_DELAY=$(( ${MAX_DAYS:-7} * 86400 ))         # latest: 7 days from now

PRIORITIES=(low normal high)
TEMPLATES=(
  "welcome|Welcome Aboard!|Welcome to our platform! Please verify your email."
  "reminder|Don't Forget!|You have an upcoming event scheduled."
  "billing|Invoice Ready|Your latest invoice is now available."
  "security|New Login Detected|We noticed a login from a new device."
  "promo|Special Offer|Enjoy 20% off your next purchase this week."
  "update|App Update|A new version is available with exciting features."
  "digest|Weekly Digest|Here is a summary of your activity this week."
)

now=$(date +%s)

for i in $(seq 1 "$COUNT"); do
  run_at=$(( now + MIN_DELAY + (RANDOM * 32768 + RANDOM) % (MAX_DELAY - MIN_DELAY) ))
  IFS='|' read -r tag title body <<< "${TEMPLATES[RANDOM % ${#TEMPLATES[@]}]}"
  priority="${PRIORITIES[RANDOM % ${#PRIORITIES[@]}]}"
  topic=$(( RANDOM % 1000 ))

  payload=$(cat <<EOF
{"topic":"$topic","body":"$body","title":"$title","priority":"$priority","tags":["$tag"]}
EOF
)

  if [[ -n "$DRY_RUN" ]]; then
    echo "curl -X PUT '$URL' -H 'Content-Type: application/json' -H 'x-task-run-at: $run_at' -d '$payload'"
  else
    curl -s -o /dev/null -w "task $i -> HTTP %{http_code} (run_at=$run_at)\n" \
      -X PUT "$URL" \
      -H "Content-Type: application/json" \
      -H "x-task-run-at: $run_at" \
      -d "$payload"
  fi
done
