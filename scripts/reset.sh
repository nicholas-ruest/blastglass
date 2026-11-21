#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); "$root/scripts/stop.sh" || true
target="$root/var/blastglass"; test "$target" = "$root/var/blastglass"; rm -rf "$target"; mkdir -p "$target"
