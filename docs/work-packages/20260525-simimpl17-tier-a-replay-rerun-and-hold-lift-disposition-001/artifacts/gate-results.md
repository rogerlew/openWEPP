# gate-results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Required SIMIMPL17 gate set executed on final package state.

## Ran
- `cargo fmt --check` -> pass (`rc=0`)
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass (`rc=0`)
- `cargo test --workspace` -> pass (`rc=0`)
- `cargo deny check` -> pass (`rc=0`)
- Logs:
- `artifacts/replay-run-20260525T072842Z/gates/gate_exit_codes.log`
- `artifacts/replay-run-20260525T072842Z/gates/fmt.stdout.log`
- `artifacts/replay-run-20260525T072842Z/gates/clippy.stdout.log`
- `artifacts/replay-run-20260525T072842Z/gates/test.stdout.log`
- `artifacts/replay-run-20260525T072842Z/gates/deny.stdout.log`
