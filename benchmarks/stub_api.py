#!/usr/bin/env python3
"""Minimal Telegram Bot API stub for the memory benchmark.

Lets both benchmark bots start their webhook servers without a real bot token:
every method call is answered with {"ok": true, "result": true}, which is enough
for setWebhook / getMe / deleteWebhook. No live traffic is served, so RSS
measured against this stub is *idle* memory, not memory under load.

Usage:
    python3 benchmarks/stub_api.py [port]     # default 8099
"""

import json
import sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8099


class Handler(BaseHTTPRequestHandler):
    def _reply(self, payload: dict) -> None:
        body = json.dumps(payload).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def _dispatch(self) -> None:
        method = self.path.rsplit("/", 1)[-1].lower()
        if method == "getme":
            self._reply(
                {
                    "ok": True,
                    "result": {
                        "id": 1,
                        "is_bot": True,
                        "first_name": "Bench",
                        "username": "bench_bot",
                        "can_join_groups": True,
                        "can_read_all_group_messages": False,
                        "supports_inline_queries": False,
                        "can_connect_to_business": False,
                        "has_main_web_app": False,
                    },
                }
            )
        elif method in ("sendmessage", "editmessagetext"):
            self._reply(
                {
                    "ok": True,
                    "result": {
                        "message_id": 1,
                        "date": 1_763_000_000,
                        "chat": {"id": 1, "type": "private", "first_name": "Bench"},
                        "text": "ok",
                    },
                }
            )
        elif method == "getwebhookinfo":
            self._reply(
                {
                    "ok": True,
                    "result": {"url": "", "has_custom_certificate": False, "pending_update_count": 0},
                }
            )
        else:
            self._reply({"ok": True, "result": True})

    def do_POST(self) -> None:  # noqa: N802 (http.server API)
        length = int(self.headers.get("Content-Length") or 0)
        if length:
            self.rfile.read(length)
        self._dispatch()

    def do_GET(self) -> None:  # noqa: N802
        length = int(self.headers.get("Content-Length") or 0)
        if length:
            self.rfile.read(length)
        self._dispatch()

    def log_message(self, *args) -> None:  # keep the benchmark output clean
        pass


if __name__ == "__main__":
    print(f"Telegram Bot API stub listening on 127.0.0.1:{PORT}", flush=True)
    ThreadingHTTPServer(("127.0.0.1", PORT), Handler).serve_forever()
