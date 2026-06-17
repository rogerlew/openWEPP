# PERFIDX03B Verification A

Ran: command-based verification after implementation.

## Verified Commands

| Command | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo test --workspace` | PASS |
| `cargo deny check` | PASS |
| `git diff --check` | PASS |
| `perf stat -e task-clock true` | PASS |

## Focused Regression Verification

| Command | Result |
| --- | --- |
| `cargo test -p openwepp-kernel-contract indexed -- --nocapture` | PASS |
| `cargo test -p openwepp-hillslope-orchestrator perfidx03b_persistent_state_refreshes_indexed_writeback_surface -- --nocapture` | PASS |
| `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_m -- --nocapture` | PASS |

## Output Verification

OFE5 same-run-name check:

- `H1.hbp`: byte-identical.
- `H1.loss.json`: byte-identical.
- `H1.wat.parquet`: byte-identical.
- `H1.plot.parquet`: byte-identical.
- `H1.pass.parquet`: logical row equality by DuckDB `EXCEPT ALL`, `only_a=0`, `only_b=0`.

