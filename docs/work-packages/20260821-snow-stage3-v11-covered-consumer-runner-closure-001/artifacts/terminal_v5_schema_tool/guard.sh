#!/usr/bin/env bash
set -euo pipefail
repo_root=$(git rev-parse --show-toplevel)
artifact_root="$repo_root/docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts"
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cargo run --quiet --manifest-path "$artifact_root/terminal_v5_schema_tool/Cargo.toml" -- "$repo_root" "$artifact_root/terminal-diagnostic-correlation-v5-schema.json" "$scratch"
for name in terminal-v5-resolved-type-graph.md terminal-v5-source-projection-matrix.md terminal-v5-generated-canonical-wire.md terminal-v5-native-wire-verification.md terminal-v5-owner-access-plan.md terminal-v5-unresolved-stale-node-report.md; do
  cmp "$scratch/$name" "$artifact_root/$name"
done
echo "terminal V5 regeneration guard: PASS"
