# gate results

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Gate summary
- SIMIMPL07 scope gate (`GAP-SIMMODE-001`): `PASS`
- SIMOUT publication closure posture: `MAINTAINED`
- SIMPIPE execution provenance posture: `MAINTAINED`

## Validation gates
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass (warnings only)

## Contract-derived targeted suite
- `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract --test simimpl04_wb13_publication_contract --test simimpl04_wepp_ui_mode_closure_contract`: pass

## Decision
- SIMIMPL07 package gate: `GO` for declared scope.
