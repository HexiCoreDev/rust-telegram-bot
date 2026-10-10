#!/usr/bin/env python3
"""Start every example against the live Bot API and assert it comes up healthy.

`cargo build --examples` proves an example *compiles*. It says nothing about
whether it can actually start: a wrong env var name, a malformed default, a
builder call that the API rejects, or a panic on the first tick all pass
compilation and fail only at runtime.

This runs each example in turn, because two examples polling the same token at
once make Telegram answer `409 Conflict`, which would look like a failure in
whichever bot lost the race.

For each example the harness reports one of:

  ok-server   still running when the deadline passed (long-polling or a
              webhook listener) -- it started and stayed up
  ok-exited   exited 0 before the deadline (one-shot demo, no server)
  FAILED      exited non-zero, or panicked, or never reached its steady state

Placeholder values are supplied for env vars that only a real Telegram account
can provide (payment provider tokens, public webhook URLs). Those are not
exercised unless a user actually triggers the feature, so a placeholder is
enough to prove startup and is clearly reported as such.

Usage:
    tools/examples_smoke.py                  # all examples
    tools/examples_smoke.py echo_bot inline_bot
    TELEGRAM_BOT_TOKEN=... tools/examples_smoke.py

Exits non-zero if any example fails.
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import signal
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXAMPLES_DIR = ROOT / "crates" / "telegram-bot" / "examples"
CARGO_MANIFEST = ROOT / "crates" / "telegram-bot" / "Cargo.toml"

# How long each example gets to reach its steady state before we judge it.
SETTLE_SECONDS = 12


@dataclass
class Example:
    name: str
    features: list[str] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    note: str = ""


def _required_features() -> dict[str, list[str]]:
    """Read `required-features` out of the [[example]] stanzas in Cargo.toml.

    Parsed rather than hardcoded so a new example cannot quietly acquire a
    feature requirement that this harness does not know about.
    """
    out: dict[str, list[str]] = {}
    text = CARGO_MANIFEST.read_text()
    for block in text.split("[[example]]")[1:]:
        name = re.search(r'name\s*=\s*"([^"]+)"', block)
        if not name:
            continue
        feats = re.search(r"required-features\s*=\s*\[([^\]]*)\]", block)
        out[name.group(1)] = (
            re.findall(r'"([^"]+)"', feats.group(1)) if feats else []
        )
    return out


# Placeholders for values that need a real Telegram account. Reported so a
# green run is never mistaken for "this feature was exercised".
PLACEHOLDERS = {
    "PAYMENT_PROVIDER_TOKEN": "placeholder-provider-token",
    "WEBHOOK_URL": "https://placeholder.invalid",
    # Numeric vars must parse, or the example dies on its own assertion and we
    # learn nothing about the code under test.
    "PORT": "18443",
    "ADMIN_CHAT_ID": "1",
    "DEVELOPER_CHAT_ID": "1",
}

# Examples that call setWebhook at startup. Telegram rejects a placeholder URL,
# so they can only be smoke-tested behind a real public tunnel -- which needs
# credentials this harness does not have. Listed explicitly so they are never
# silently counted as passing.
NEEDS_PUBLIC_URL = {"webhook_bot", "custom_webhook_bot", "bench_bot", "webhook_echo_bot"}

REQUIRED_FEATURES = _required_features()


def discover() -> list[Example]:
    examples = []
    for path in sorted(EXAMPLES_DIR.glob("*.rs")):
        name = path.stem
        src = path.read_text()
        envs = sorted(set(re.findall(r'env::var\("([A-Z_]+)"\)', src)))
        examples.append(
            Example(
                name=name,
                features=REQUIRED_FEATURES.get(name, []),
                env={e: PLACEHOLDERS.get(e, "") for e in envs},
            )
        )
    return examples


def telegram_call(token: str, method: str, **params) -> dict:
    """Call a Bot API method. Returns the parsed envelope, or {} on failure."""
    data = urllib.parse.urlencode(params).encode()
    req = urllib.request.Request(
        f"https://api.telegram.org/bot{token}/{method}",
        data=data,
        headers={"Content-Type": "application/x-www-form-urlencoded"},
    )
    try:
        with urllib.request.urlopen(req, timeout=20) as resp:
            import json

            return json.loads(resp.read())
    except (urllib.error.URLError, TimeoutError, ValueError):
        return {}


def clear_webhook(token: str) -> None:
    """A leftover webhook makes every polling example fail with 409."""
    telegram_call(token, "deleteWebhook", drop_pending_updates="false")


@dataclass
class Tunnel:
    """A live public HTTPS URL in front of a local port."""

    label: str
    base_url: str
    proc: subprocess.Popen
    share_token: str = ""


def _available_tunnels() -> list[str]:
    return [name for name in TUNNEL_COMMANDS if shutil.which(name)]


def _start_cloudflared(port: int, timeout: int = 90) -> Tunnel | None:
    proc = subprocess.Popen(
        ["cloudflared", "tunnel", "--url", f"http://127.0.0.1:{port}",
         "--no-autoupdate"],
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
        start_new_session=True,
    )
    deadline = time.time() + timeout
    url = ""
    while time.time() < deadline:
        line = proc.stdout.readline() if proc.stdout else ""
        if not line:
            if proc.poll() is not None:
                return None
            continue
        m = re.search(r"https://([a-z0-9-]+\.trycloudflare\.com)", line)
        if m:
            url = f"https://{m.group(1)}"
            break
    if not url:
        _kill_tree(proc)
        return None
    # The tunnel needs a moment before it serves; the caller retries anyway.
    return Tunnel(label="cloudflared", base_url=url, proc=proc)


def _start_zrok(port: int, attempts: int = 3) -> Tunnel | None:
    last = ""
    for attempt in range(attempts):
        # zrok2 re-execs itself as a `share public --subordinate` child, so the
        # process we spawn is only the parent. start_new_session puts the pair
        # in its own process group; without killing the group, every run leaks
        # a child until the account hits its share limit.
        proc = subprocess.Popen(
            ["zrok2", "share", "public", "--headless", f"http://127.0.0.1:{port}"],
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
            start_new_session=True,
        )
        seen: list[str] = []
        deadline = time.time() + 60
        while time.time() < deadline:
            line = proc.stdout.readline() if proc.stdout else ""
            if not line:
                if proc.poll() is not None:
                    break
                continue
            seen.append(line.strip())
            share = re.search(r'token:"([^"]+)"', line)
            endpoint = re.search(r'frontendEndpoints:"([^"]+)"', line)
            if share and endpoint:
                # The share is registered with the controller, which keeps it
                # serving through the local zrok agent, so the CLI exiting here
                # is fine; cleanup is `zrok2 delete share <token>`.
                return Tunnel(
                    label="zrok",
                    base_url=f"https://{endpoint.group(1)}",
                    proc=proc,
                    share_token=share.group(1),
                )
        _kill_tree(proc)
        last = " | ".join(seen) or "no output"
        if attempt < attempts - 1:
            time.sleep(10)
    print(f"      (zrok unavailable: {last})", file=sys.stderr)
    return None


TUNNEL_COMMANDS = ("cloudflared", "zrok2")


def start_tunnel(port: int) -> tuple[Tunnel | None, str]:
    """Open a public HTTPS URL onto 127.0.0.1:<port>.

    Prefers cloudflared, then zrok. Both produce a URL Telegram accepts;
    trying more than one keeps a throttled provider from blocking the run.
    """
    tried = _available_tunnels()
    if not tried:
        return None, "no tunnel provider installed (need cloudflared or zrok2)"

    for name in TUNNEL_COMMANDS:
        if name not in tried:
            continue
        tunnel = _start_cloudflared(port) if name == "cloudflared" else _start_zrok(port)
        if tunnel is not None:
            return tunnel, ""
    return None, f"could not open a tunnel (tried: {', '.join(tried)})"


def _kill_tree(proc: subprocess.Popen) -> None:
    """Terminate a process and any children it re-exec'd into."""
    if proc.poll() is not None:
        return
    try:
        os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
    except (ProcessLookupError, PermissionError):
        proc.terminate()
    try:
        proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        try:
            os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
        except (ProcessLookupError, PermissionError):
            proc.kill()
        proc.wait(timeout=10)


def stop_tunnel(tunnel: Tunnel) -> None:
    if tunnel.share_token:
        subprocess.run(
            ["zrok2", "delete", "share", tunnel.share_token],
            cwd=ROOT, capture_output=True, timeout=90,
        )
    _kill_tree(tunnel.proc)


def wait_for_http(url: str, timeout: int = 30) -> bool:
    """Poll until the URL answers anything at all (even 403 proves liveness)."""
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            urllib.request.urlopen(url, timeout=5)
            return True
        except urllib.error.HTTPError:
            return True  # a status code means the server is up
        except (urllib.error.URLError, TimeoutError):
            time.sleep(1)
    return False


def warm_tunnel(url: str, timeout: int = 150) -> bool:
    """Wait until the tunnel itself is live, before the bot binds its port.

    Telegram verifies a webhook registration by sending a request to the URL.
    A freshly created tunnel hostname can take a while to appear in DNS --
    cloudflared prints the URL before the record resolves, and a lookup that
    fails once is not fatal, just early -- so probe patiently. Telegram gives
    up on a slow first response, so probing first turns a flaky registration
    into a deterministic one.

    Any HTTP status counts, including the 502 a tunnel returns while nothing is
    listening locally yet: that still proves the public path works.
    """
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            urllib.request.urlopen(url, timeout=10)
            return True
        except urllib.error.HTTPError:
            return True
        except (urllib.error.URLError, TimeoutError, OSError):
            time.sleep(3)
    return False


def run_tunnelled(ex: Example, token: str, settle: int) -> tuple[str, str]:
    """Run a webhook example behind a real public URL and confirm Telegram
    accepted the registration."""
    port = int(PLACEHOLDERS.get("PORT", "18443"))
    tunnel, reason = start_tunnel(port)
    if tunnel is None:
        return "skipped", reason

    try:
        # Warm the public path before the bot registers, so Telegram's
        # verification probe does not race the tunnel's first route.
        if not warm_tunnel(f"{tunnel.base_url}/healthcheck"):
            return "FAILED", "tunnel never became reachable"

        env = dict(os.environ)
        env["TELEGRAM_BOT_TOKEN"] = token
        env["RUST_LOG"] = env.get("RUST_LOG", "warn")
        env["WEBHOOK_URL"] = tunnel.base_url
        env["PORT"] = str(port)
        for key in ex.env:
            if key in ("WEBHOOK_URL", "TELEGRAM_BOT_TOKEN"):
                continue
            # Only inject values we actually have. Setting a var to "" is worse
            # than leaving it unset: an example's `env::var(X).unwrap_or(default)`
            # sees Ok("") and silently drops its own default.
            if key in PLACEHOLDERS:
                env[key] = PLACEHOLDERS[key]
            else:
                env.pop(key, None)

        clear_webhook(token)
        cmd = ["cargo", "run", "--offline", "--quiet", "-p", "rust-tg-bot",
               "--example", ex.name]
        if ex.features:
            cmd += ["--features", ",".join(ex.features)]

        proc = subprocess.Popen(
            cmd, cwd=ROOT, env=env,
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
            start_new_session=True,
        )
        deadline = time.time() + settle
        while time.time() < deadline and proc.poll() is None:
            time.sleep(0.25)

        if proc.poll() is not None:
            out = proc.stdout.read() if proc.stdout else ""
            return ("ok-exited" if proc.returncode == 0 else "FAILED"), out

        # Still up. Now the part that matters: did Telegram actually register
        # the webhook against our public URL? A bot that merely starts is not
        # proof the tunnel works. Retry, because the registration and this read
        # race each other, and a transient API hiccup must not read as failure.
        url = ""
        for _ in range(6):
            time.sleep(3)
            info = telegram_call(token, "getWebhookInfo")
            url = info.get("result", {}).get("url", "")
            if url.startswith(tunnel.base_url):
                break
        if not url.startswith(tunnel.base_url):
            _kill_tree(proc)
            return "FAILED", f"Telegram did not accept the webhook URL (got {url!r})"

        reachable = wait_for_http(f"{tunnel.base_url}/healthcheck", timeout=30)
        _kill_tree(proc)

        if not reachable:
            return "FAILED", "webhook registered but the public URL never answered"
        return "ok-server", f"registered {url} via {tunnel.label}"
    finally:
        clear_webhook(token)
        stop_tunnel(tunnel)


def run_one(ex: Example, token: str, settle: int) -> tuple[str, str]:
    """Run one example and classify the outcome."""
    env = dict(os.environ)
    env["TELEGRAM_BOT_TOKEN"] = token
    env["RUST_LOG"] = env.get("RUST_LOG", "warn")
    for key in ex.env:
        # Only supply what we actually have. Injecting a generic string into
        # an unrelated var produces nonsense values (a URL path of "placeholder")
        # and tests nothing; setting "" is worse still, because an example's
        # `env::var(X).unwrap_or(default)` sees Ok("") and drops its default.
        if key == "TELEGRAM_BOT_TOKEN":
            continue
        if key in PLACEHOLDERS:
            env[key] = PLACEHOLDERS[key]
        else:
            env.pop(key, None)

    cmd = [
        "cargo", "run", "--offline", "--quiet",
        "-p", "rust-tg-bot", "--example", ex.name,
    ]
    if ex.features:
        cmd += ["--features", ",".join(ex.features)]

    clear_webhook(token)

    proc = subprocess.Popen(
        cmd, cwd=ROOT, env=env,
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
        start_new_session=True,
    )
    deadline = time.time() + settle
    output = ""
    while time.time() < deadline:
        if proc.poll() is not None:
            break
        time.sleep(0.25)

    if proc.poll() is None:
        # Still alive: it started and stayed up.
        _kill_tree(proc)
        return "ok-server", output

    code = proc.returncode
    output = proc.stdout.read() if proc.stdout else ""
    if code == 0:
        return "ok-exited", output
    return "FAILED", output


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("names", nargs="*", help="examples to run (default: all)")
    ap.add_argument("--settle", type=int, default=SETTLE_SECONDS)
    ap.add_argument(
        "--no-tunnel",
        action="store_true",
        help="skip the webhook examples instead of opening a real zrok tunnel",
    )
    args = ap.parse_args()

    token = os.environ.get("TELEGRAM_BOT_TOKEN", "").strip()
    if not token:
        print("TELEGRAM_BOT_TOKEN is required for the live smoke test.", file=sys.stderr)
        return 2

    examples = discover()
    if args.names:
        wanted = set(args.names)
        examples = [e for e in examples if e.name in wanted]
        missing = wanted - {e.name for e in examples}
        if missing:
            print(f"no such example(s): {', '.join(sorted(missing))}", file=sys.stderr)
            return 2

    print(f"smoke-testing {len(examples)} example(s) against the live Bot API\n")
    results = []
    for ex in examples:
        feats = f" [{','.join(ex.features)}]" if ex.features else ""
        if ex.name in NEEDS_PUBLIC_URL:
            if args.no_tunnel:
                print(f"  {ex.name}{feats} ... SKIPPED (--no-tunnel)")
                results.append((ex, "skipped", ""))
                continue
            print(f"  {ex.name}{feats} ... ", end="", flush=True)
            status, output = run_tunnelled(ex, token, args.settle)
            print(status)
            results.append((ex, status, output))
            time.sleep(1)
            continue
        print(f"  {ex.name}{feats} ... ", end="", flush=True)
        status, output = run_one(ex, token, args.settle)
        print(status)
        results.append((ex, status, output))
        time.sleep(1)  # let Telegram settle between runs

    clear_webhook(token)

    failures = [(e, o) for e, s, o in results if s == "FAILED"]
    skipped = [(e, o) for e, s, o in results if s == "skipped"]
    tested = len(results) - len(skipped)
    print(f"\n{tested - len(failures)}/{tested} healthy ({len(skipped)} skipped)")
    if skipped:
        print("skipped:")
        for e, reason in skipped:
            print(f"  - {e.name}: {reason}")
    if failures:
        print("\nfailures:")
        for ex, output in failures:
            print(f"\n--- {ex.name} ---")
            tail = "\n".join(output.strip().splitlines()[-15:])
            print(tail or "(no output)")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
