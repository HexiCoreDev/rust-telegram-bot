#!/usr/bin/env bash
# Offline memory benchmark: idle and post-load RSS for the benchmark bots.
#
# Unlike run_benchmark.sh this needs no bot token and no public tunnel: both bots
# are pointed at benchmarks/stub_api.py, which answers every Bot API call with a
# canned "ok" response. Synthetic updates are POSTed directly to each bot's
# webhook, so the same handler paths run as in the live benchmark.
#
# Usage: benchmarks/measure_rss.sh [updates]      (default 20 updates per bot)
#
# Result: "idle" is RSS after startup (webhook registered, no traffic);
#         "load" is RSS after the synthetic updates were processed.

set -uo pipefail

UPDATES="${1:-20}"
STUB_PORT="${STUB_PORT:-8099}"
APP_PORT="${APP_PORT:-8000}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STUB_PID=""

cleanup() {
  if [ -n "$STUB_PID" ] && kill -0 "$STUB_PID" 2>/dev/null; then kill "$STUB_PID" 2>/dev/null; fi
  fuser -k "$APP_PORT/tcp" 2>/dev/null
  sleep 0.5
}
trap cleanup EXIT

rss_mb() { # $1 = pid
  awk '/VmRSS/ {printf "%.1f", $2/1024}' "/proc/$1/status" 2>/dev/null
}

payload() {
  local i="$1"
  printf '{"update_id":%d,"message":{"message_id":%d,"date":1763000000,"chat":{"id":1,"type":"private","first_name":"Bench"},"from":{"id":1,"is_bot":false,"first_name":"Bench"},"text":"hello %d"}}' "$i" "$i" "$i"
}

run_bot() { # $1=label $2=binary $3=webhook path $4=extra header (optional)
  local label="$1" bin="$2" path="$3" header="${4:-}"
  echo ""
  echo "========================================="
  echo "  $label"
  echo "========================================="

  port_guard() { fuser -k "$APP_PORT/tcp" 2>/dev/null; sleep 0.5; }
  port_guard

  if [ ! -x "$bin" ]; then
    echo "SKIP: $bin not built"
    return
  fi

  TELEGRAM_BOT_TOKEN="123456:AAxxBBccDDeeFFgg" \
  TELOXIDE_TOKEN="123456:AAxxBBccDDeeFFgg" \
  WEBHOOK_URL="http://127.0.0.1:$APP_PORT" \
  TELEGRAM_API_URL="http://127.0.0.1:$STUB_PORT" \
  RUST_LOG=warn \
    "$bin" >/tmp/"$label".log 2>&1 &
  local pid=$!
  sleep 7

  if ! kill -0 "$pid" 2>/dev/null; then
    echo "FAILED to start; log:"
    tail -5 /tmp/"$label".log
    return
  fi

  local idle; idle=$(rss_mb "$pid")
  echo "idle RSS:  $idle MB"

  local ok=0
  for i in $(seq 1 "$UPDATES"); do
    if [ -n "$header" ]; then
      curl -sS -o /dev/null -H "Content-Type: application/json" -H "$header" \
        -d "$(payload "$i")" "http://127.0.0.1:$APP_PORT$path" && ok=$((ok + 1))
    else
      curl -sS -o /dev/null -H "Content-Type: application/json" \
        -d "$(payload "$i")" "http://127.0.0.1:$APP_PORT$path" && ok=$((ok + 1))
    fi
  done
  sleep 3

  local load; load=$(rss_mb "$pid")
  echo "updates:   $ok / $UPDATES accepted"
  echo "load RSS:  $load MB"
  echo "growth:    $(awk -v a="$idle" -v b="$load" 'BEGIN {printf "%.1f", b - a}') MB"

  kill "$pid" 2>/dev/null
  wait "$pid" 2>/dev/null
}

python3 "$ROOT/benchmarks/stub_api.py" "$STUB_PORT" >/tmp/stub_api.log 2>&1 &
STUB_PID=$!
sleep 1
echo "Bot API stub: $(head -1 /tmp/stub_api.log)"

run_bot "teloxide-bench" "$ROOT/benchmarks/teloxide/target/release/teloxide-bench" "/"
run_bot "rtb-bench" "$ROOT/target/release/examples/bench_bot" "/telegram" "X-Telegram-Bot-Api-Secret-Token: bench-secret-token"

echo ""
echo "done (logs: /tmp/teloxide-bench.log, /tmp/rtb-bench.log)"
