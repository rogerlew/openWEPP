#!/usr/bin/env bash
set -euo pipefail
repo_root=$(git rev-parse --show-toplevel)
artifact_root="$repo_root/docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts"
target_root=$(nix develop --command bash -c 'printf "\n%s\n" "$CARGO_TARGET_DIR"' | tail -n 1)
nix develop --command cargo rustdoc -p openwepp-hillslope-orchestrator --lib -- --document-private-items
doc_root="$target_root/doc/openwepp_hillslope_orchestrator"
toolchain=$(nix develop --command rustc -Vv | tr '\n' ';')
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
nix develop --command cargo run --quiet --manifest-path "$artifact_root/terminal_v6_compiler_index_tool/Cargo.toml" -- generate "$artifact_root/terminal-diagnostic-correlation-v6-schema.json" "$doc_root" "$scratch" "$toolchain"
nix develop --command cargo run --quiet --manifest-path "$artifact_root/terminal_v6_compiler_index_tool/Cargo.toml" -- negative-fixtures "$artifact_root/terminal-diagnostic-correlation-v6-schema.json" "$doc_root"
for name in terminal-v6-compiler-bindings.md terminal-v6-dto-graph.md terminal-v6-carrier-projection.md terminal-v6-owner-access-plan.md terminal-v6-evidence-sufficiency-matrix.md terminal-v6-calculated-resolution-report.md; do
  cmp "$scratch/$name" "$artifact_root/$name"
done
echo "terminal V6 compiler-index regeneration guard: PASS"
