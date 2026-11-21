#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); runtime="$root/.runtime"; mkdir -p "$runtime" "$root/var/blastglass"
"$root/scripts/stop.sh" >/dev/null 2>&1 || true
export PATH="$HOME/.cargo/bin:/usr/bin:$PATH" PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-${XDG_CACHE_HOME:-$HOME/.cache}/ms-playwright}"
export PORT=${BLASTGLASS_PORT:-8787} BLASTGLASS_DATA_PATH=${BLASTGLASS_DATA_PATH:-$root/var/blastglass/evidence.jsonl}
export BLASTGLASS_RUVECTOR_CMD="$root/scripts/upstream-runtime/run.sh ruvector"
export BLASTGLASS_FANN_CMD="$root/scripts/upstream-runtime/run.sh fann"
export BLASTGLASS_MIDSTREAM_CMD="$root/scripts/upstream-runtime/run.sh midstream"
export BLASTGLASS_QUDAG_CMD="$root/scripts/upstream-runtime/run.sh qudag"
export BLASTGLASS_FEDERATION_CMD="$root/scripts/upstream-runtime/run.sh federation"
export BLASTGLASS_OPA_CMD="$root/scripts/upstream-runtime/run.sh opa"
export BLASTGLASS_NATS_CMD="$root/scripts/upstream-runtime/run.sh nats"
setsid sh -c "cd '$root' && exec cargo run --locked -p blastglass-service" >"$runtime/backend.log" 2>&1 & echo $! >"$runtime/backend.pid"
setsid sh -c "cd '$root/frontend' && exec npm run dev -- --port 4173 --strictPort" >"$runtime/frontend.log" 2>&1 & echo $! >"$runtime/frontend.pid"
"$root/scripts/readiness.sh"
echo "Blastglass ready at http://127.0.0.1:4173"
