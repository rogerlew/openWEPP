#!/usr/bin/env bash
set -euo pipefail
repo_root="$(git rev-parse --show-toplevel)"
tool_dir="$repo_root/docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts/terminal_v4_census_tool"
expected="$repo_root/docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts/terminal-v4-live-type-census.md"
actual="$(mktemp)"
trap 'rm -f "$actual"' EXIT
cargo run --quiet --manifest-path "$tool_dir/Cargo.toml" -- "$repo_root" "$actual"
cmp "$expected" "$actual"
echo "terminal V4 live-type census guard: PASS"
