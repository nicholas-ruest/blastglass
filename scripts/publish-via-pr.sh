#!/usr/bin/env bash
set -euo pipefail
owner=nicholas-ruest; repo=blastglass; branch=product/blastglass-2026-11-20
test "$(gh api user --jq .login)" = "$owner" || { echo PUBLISH_DENIED:wrong-owner >&2; exit 71; }
if gh repo view "$owner/$repo" >/dev/null 2>&1; then echo PUBLISH_DENIED:repository-exists >&2; exit 72; fi
test -z "$(git status --porcelain)"; source_sha=$(git rev-parse HEAD); base_sha=$(git rev-parse HEAD^)
gh repo create "$owner/$repo" --public --description "Evidence-bearing operational resilience rehearsal studio"
remote="https://github.com/$owner/$repo.git"; git remote add origin "$remote"
git push origin "$base_sha:refs/heads/main"; git push origin "$source_sha:refs/heads/$branch"
gh pr create --repo "$owner/$repo" --base main --head "$branch" --title "Build Blastglass complete product" --body "Complete React, typed API, executable middleware, 25-crate Rust backend, seven source-pinned integrations, restart-safe audit evidence, containers and three browser journeys. Human merge required."
printf '{"owner":"%s","repository":"%s","source_sha":"%s","branch":"%s"}\n' "$owner" "$repo" "$source_sha" "$branch"
