#!/bin/bash
# Memory benchmark runner for PTB vs teloxide vs RTB
# Usage: ./run_benchmark.sh <ptb|teloxide|rtb> [duration_seconds]
#
# Prerequisites:
#   - TELEGRAM_BOT_TOKEN and WEBHOOK_URL env vars set
#   - PTB: pip install "python-telegram-bot[webhooks]" starlette uvicorn
#   - teloxide: cargo build --release in benchmarks/teloxide/
#   - RTB: cargo build --release in workspace root

set -e

FRAMEWORK="${1:?Usage: $0 <ptb|teloxide|rtb> [duration_seconds]}"
DURATION="${2:-30}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

: "${TELEGRAM_BOT_TOKEN:?Set TELEGRAM_BOT_TOKEN}"
: "${WEBHOOK_URL:?Set WEBHOOK_URL}"

echo "========================================="
echo "  Memory Benchmark: ${FRAMEWORK}"
echo "========================================="
echo ""

# Kill anything on port 8000
fuser -k 8000/tcp 2>/dev/null || true
sleep 1

case "$FRAMEWORK" in
  ptb)
    echo "Starting PTB bot..."
    python3 "$SCRIPT_DIR/ptb/bench_bot.py" &
    BOT_PID=$!
    PROC_NAME="python3"
    ;;
  teloxide)
    echo "Starting teloxide bot..."
    cd "$SCRIPT_DIR/teloxide"
    cargo build --release 2>/dev/null
    TELOXIDE_TOKEN="$TELEGRAM_BOT_TOKEN" ./target/release/teloxide-bench &
    BOT_PID=$!
    PROC_NAME="teloxide-bench"
    cd "$SCRIPT_DIR"
    ;;
  rtb)
    echo "Starting RTB bot..."
    export ADMIN_CHAT_ID="${ADMIN_CHAT_ID:-0}"
    # Use the workspace release binary
    cd "$SCRIPT_DIR/.."
    cargo build --release -p rust-tg-bot --example custom_webhook_bot --features webhooks 2>/dev/null
    ./target/release/examples/custom_webhook_bot &
    BOT_PID=$!
    PROC_NAME="custom_webhook"
    cd "$SCRIPT_DIR"
    ;;
  *)
    echo "Unknown framework: $FRAMEWORK"
    echo "Usage: $0 <ptb|teloxide|rtb>"
    exit 1
    ;;
esac

sleep 5

# Find actual bot process (not bash wrapper)
ACTUAL_PID=$(ps -eo pid,comm | grep "$PROC_NAME" | grep -v grep | tail -1 | awk '{print $1}')

if [ -z "$ACTUAL_PID" ]; then
    echo "ERROR: Bot process not found!"
    kill $BOT_PID 2>/dev/null
    exit 1
fi

echo ""
echo "Bot PID: $ACTUAL_PID"
echo "Measuring idle memory..."
IDLE_RSS=$(ps -o rss= -p "$ACTUAL_PID" | tr -d ' ')
echo "Idle RSS: ${IDLE_RSS} KB ($((IDLE_RSS / 1024)) MB)"

echo ""
echo ">>> NOW SEND ~20 messages + button presses to the bot <<<"
echo ">>> You have ${DURATION} seconds... <<<"
echo ""
sleep "$DURATION"

echo "Measuring post-load memory..."
LOAD_RSS=$(ps -o rss= -p "$ACTUAL_PID" | tr -d ' ')
LOAD_VSZ=$(ps -o vsz= -p "$ACTUAL_PID" | tr -d ' ')

echo ""
echo "========================================="
echo "  RESULTS: ${FRAMEWORK}"
echo "========================================="
echo "Idle RSS:  ${IDLE_RSS} KB ($((IDLE_RSS / 1024)) MB)"
echo "Load RSS:  ${LOAD_RSS} KB ($((LOAD_RSS / 1024)) MB)"
echo "VSZ:       ${LOAD_VSZ} KB ($((LOAD_VSZ / 1024)) MB)"
echo "Growth:    $(( (LOAD_RSS - IDLE_RSS) )) KB"
echo ""

kill "$ACTUAL_PID" 2>/dev/null
kill $BOT_PID 2>/dev/null
wait 2>/dev/null
