# Implementation and Test Evidence

Status: `executed`

Evidence mode: `static + ran`

Implementation summary:

- Removed watershed request/writeback runtime types, trait, dispatch entrypoints,
  compatibility surface builders, harvest helpers, and obsolete tests.
- Kept generic/hillslope writeback infrastructure outside W5 scope.
- Kept direct watershed execution on typed `WatershedNetworkFrame`.
- Preserved WS12 non-finite/domain guard taxonomy for parser-projected
  impoundment coefficients.
- Trimmed stale runtime-input error variants tied to deleted chan.inp/channel
  surface builders.
- Expanded typed W5 coverage to restore WS11 branch, WS18/WS20 capacity, WS12
  active/inactive, and WS12 guard-family checks.

Focused gates run:

```text
cargo check -p openwepp-watershed-orchestrator -p openwepp-runner --tests
PASS

cargo fmt --check
PASS

cargo nextest run --test wshedw5_typed_watershed_runtime_contract
8 passed

cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw5_public_cli_uses_typed_network_and_publication_frames
1 passed

cargo clippy --workspace --all-targets -- -D warnings
PASS
```

Final full-suite gate is recorded in `gate-results.md`.
