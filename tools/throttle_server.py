#!/usr/bin/env python3
"""Throttled test server for benchmarking OmniDL segmentation.

Serves a synthetic file over HTTP with Range support, capping each
connection at --per-conn-kbps so multi-connection speedups are visible:

    python3 tools/throttle_server.py --size-mb 64 --per-conn-kbps 2048
    omnidl http://127.0.0.1:8765/file.bin -c 8

Then compare -c 1 vs -c 8 wall time.
"""
import argparse
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import re
import time

SIZE = 0
PER_CONN_BPS = 0


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *a):  # quiet
        pass

    def _send_range(self, start, end):
        length = end - start + 1
        self.send_response(206)
        self.send_header("Content-Type", "application/octet-stream")
        self.send_header("Accept-Ranges", "bytes")
        self.send_header("Content-Range", f"bytes {start}-{end}/{SIZE}")
        self.send_header("Content-Length", str(length))
        self.end_headers()
        chunk = bytes(8192)
        sent = 0
        t0 = time.time()
        while sent < length:
            n = min(len(chunk), length - sent)
            self.wfile.write(chunk[:n])
            sent += n
            # per-connection throttle
            expected = sent / PER_CONN_BPS
            dt = t0 + expected - time.time()
            if dt > 0:
                time.sleep(dt)

    def do_GET(self):
        m = re.match(r"bytes=(\d+)-(\d*)", self.headers.get("Range", ""))
        if m:
            start = int(m.group(1))
            end = int(m.group(2)) if m.group(2) else SIZE - 1
            self._send_range(start, min(end, SIZE - 1))
        else:
            self._send_range(0, SIZE - 1)

    def do_HEAD(self):
        self.send_response(200)
        self.send_header("Accept-Ranges", "bytes")
        self.send_header("Content-Length", str(SIZE))
        self.end_headers()


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=8765)
    ap.add_argument("--size-mb", type=int, default=64)
    ap.add_argument("--per-conn-kbps", type=int, default=2048)
    a = ap.parse_args()
    SIZE = a.size_mb * 1024 * 1024
    PER_CONN_BPS = a.per_conn_kbps * 1024
    print(f"serving {a.size_mb} MiB on :{a.port}, {a.per_conn_kbps} KiB/s per connection")
    ThreadingHTTPServer(("127.0.0.1", a.port), Handler).serve_forever()
