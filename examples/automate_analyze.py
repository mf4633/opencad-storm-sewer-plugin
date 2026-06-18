#!/usr/bin/env python3
"""Minimal Storm Sewer workflow over Open CAD Studio --serve (no PyO3).

Requires: OpenCADStudio v0.6.0+ with --serve, and this plugin installed.

Usage:
  python examples/automate_analyze.py plan.dwg
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

# Reuse the upstream thin client when available; otherwise inline a tiny subset.
try:
    sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "OpenCADStudio" / "docs" / "automation"))
    from ocs import Ocs  # type: ignore
except ImportError:
    class Ocs:
        def __init__(self, binary: str = "OpenCADStudio"):
            self.binary = binary
            self._proc = None

        def __enter__(self):
            self._proc = subprocess.Popen(
                [self.binary, "--serve"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1,
            )
            self._proc.stdout.readline()
            return self

        def __exit__(self, *args):
            if self._proc:
                self._proc.stdin.close()
                self._proc.wait(timeout=10)

        def _send(self, req: dict) -> dict:
            assert self._proc and self._proc.stdin and self._proc.stdout
            self._proc.stdin.write(json.dumps(req) + "\n")
            self._proc.stdin.flush()
            line = self._proc.stdout.readline()
            return json.loads(line)

        def open(self, path: str) -> dict:
            return self._send({"op": "open", "path": path})

        def run(self, cmd: str) -> dict:
            return self._send({"op": "run", "cmd": cmd})

        def query(self, **filters) -> dict:
            return self._send({"op": "query", **filters})

        def save(self, path: str) -> dict:
            return self._send({"op": "save", "path": path})


def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: automate_analyze.py <drawing.dwg>", file=sys.stderr)
        return 2

    dwg = sys.argv[1]
    with Ocs() as ocs:
        print("open:", ocs.open(dwg))
        print("analyze:", ocs.run("SS_ANALYZE"))
        structs = ocs.query(type="Circle")
        pipes = ocs.query(type="Line")
        print(f"structures (circles): {structs.get('total', len(structs.get('entities', [])))}")
        print(f"lines (incl. pipes): {pipes.get('total', len(pipes.get('entities', [])))}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())