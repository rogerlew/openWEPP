# gate results

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Gate summary
- SIMIMPL05 scope gate (`GAP-SIMPIPE-001`): `PASS`
- SIMIMPL06 prerequisite posture (`GAP-SIMOUT-001`): `DEFERRED`
- SIMIMPL07 prerequisite posture (`GAP-SIMMODE-001`): `DEFERRED`

## Validation gates
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass (warnings only)

## Deferred expected-fail checks
- `simimpl04_wepp_ui_mode_closure_contract -- --ignored`: expected fail
- `simimpl04_wb13_publication_contract -- --ignored`: expected fail

## Decision
- SIMIMPL05 package gate: `GO` for declared scope.
