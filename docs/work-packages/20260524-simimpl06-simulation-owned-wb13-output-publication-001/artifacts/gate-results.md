# gate results

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Gate summary
- SIMIMPL06 scope gate (`GAP-SIMOUT-001`): `PASS`
- SIMIMPL07 prerequisite posture (`GAP-SIMMODE-001`): `DEFERRED`
- SIMIMPL08 prerequisite posture (`GAP-SIMCONS-001`): `DEFERRED`

## Validation gates
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass (warnings only)

## Deferred expected-fail checks
- `simimpl04_wepp_ui_mode_closure_contract -- --ignored`: expected fail

## Decision
- SIMIMPL06 package gate: `GO` for declared scope.
