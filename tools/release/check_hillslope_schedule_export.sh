#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

cd "$repo_root"

cargo run --quiet \
  --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml \
  --bin openwepp_hillslope_schedule_export \
  -- generate --output-dir "$tmp_dir"

for artifact in \
  hillslope-phase-schedule.json \
  hillslope-phase-schedule.mmd \
  hillslope-phase-schedule.dot
do
  expected="docs/architecture/generated/$artifact"
  actual="$tmp_dir/$artifact"
  if ! diff -u "$expected" "$actual"; then
    echo "hillslope schedule export drift detected for $artifact" >&2
    echo "Regenerate with:" >&2
    echo "  cargo run --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --bin openwepp_hillslope_schedule_export -- generate --output-dir docs/architecture/generated" >&2
    exit 1
  fi
done

echo "hillslope schedule export artifacts are congruent"
