#!/usr/bin/env bash
set -euo pipefail
curl -fsS -X POST http://127.0.0.1:8787/api/v1/rehearsals -H 'content-type: application/json' -H 'authorization: Bearer blastglass-operator' -H 'x-operator: seed@example.com' -H 'x-role: rehearsal-operator' --data '{"service":"inventory-api","scenario":"regional dependency loss","blast_radius":32,"recovery_minutes":14}'
