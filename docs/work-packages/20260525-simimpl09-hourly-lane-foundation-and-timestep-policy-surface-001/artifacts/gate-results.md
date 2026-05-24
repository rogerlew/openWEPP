# gate-results

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Gate summary
- SIMIMPL09 scope gate (`GAP-SIMMODE-001` + `GAP-SIMCONS-001`): `PASS`
- Hourly lane typed timestep-policy closure: `PASS`
- Adapter-boundary adopt-only closure: `PASS`

## Validation gates
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass (warnings only)

## Targeted contract-derived suite
- `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract --test simimpl04_wb13_publication_contract --test simimpl04_wepp_ui_mode_closure_contract`: pass

## `cargo deny check` warnings (non-blocking)
- duplicate lockfile entries (`getrandom`, `hashbrown`, `twox-hash`)
- unmatched allow-list licenses (`ISC`, `Unicode-DFS-2016`)

## Decision
- SIMIMPL09 package gate: `GO` for declared scope.
