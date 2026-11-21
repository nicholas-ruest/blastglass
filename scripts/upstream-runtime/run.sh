#!/usr/bin/env bash
set -euo pipefail
slug=${1:?upstream slug required}
root=${BLASTGLASS_UPSTREAM_ROOT:?exact-pinned upstream root required}
export CARGO_TARGET_DIR=${BLASTGLASS_UPSTREAM_TARGET:-/home/ruv/.cache/blastglass-upstream-target}
verify(){ test -d "$root/$1/.git"; test "$(git -C "$root/$1" rev-parse HEAD)" = "$2"; }
case "$slug" in
  ruvector) verify ruvector 979c31acc57b970c1a7e0771ed527c1703c137e8; cargo test --quiet --manifest-path "$root/ruvector/Cargo.toml" -p ruvector-core --test filtered_search -- --nocapture >/dev/null; echo similarity_set ;;
  fann) verify ruv-fann 02431fd3b3d5639f38feee8f964ec9a8bcc27fc5; cargo test --quiet --manifest-path "$root/ruv-fann/Cargo.toml" reference_ranks_and_deterministic_sorted_intervals -- --nocapture >/dev/null; echo calibrated_interval ;;
  midstream) verify midstream 0934fd3b2f92b4abf93d0d56e6d9085a8d4ad92e; cargo test --quiet --manifest-path "$root/midstream/Cargo.toml" -p midstream test_metric_ingestion -- --nocapture >/dev/null; echo safety_window ;;
  qudag) verify qudag b53466814554737c93f916d458783f7dff6c8acd; cargo test --quiet --manifest-path "$root/qudag/Cargo.toml" -p qudag-network test_ml_kem_encapsulation_decapsulation -- --nocapture >/dev/null; echo kem_ciphertext ;;
  federation) verify federated-mcp 1caf7ace9e8c6977e1c0d77c89989d9ae6a35f71; node --test --test-name-pattern 'official HTTP lifecycle' "$root/federated-mcp/modern/test/gateway.test.mjs" >/dev/null; echo federated_receipt ;;
  opa) verify opa ba39415db4b3c90cd68942a74b3cc2223ec2b23c; test "$("$root/opa/opa" eval --fail -f raw -d "$(dirname "$0")/policy.rego" -i "$(dirname "$0")/allow.json" 'data.blastglass.allow')" = true; echo policy_allow ;;
  nats) verify nats-server 2a9066eb57ff4aa50d8e6a979bdfbd6a8b9db251; node "$(dirname "$0")/nats.mjs" "$root/nats-server/nats-server" ;;
  *) echo "unknown upstream: $slug" >&2; exit 64 ;;
esac
