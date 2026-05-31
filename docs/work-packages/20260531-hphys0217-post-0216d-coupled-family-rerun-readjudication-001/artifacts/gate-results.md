# HPHYS0217 Gate Results

Status: completed
Evidence mode: Ran

## Commands run
1. `cargo build -p openwepp-runner --bin openwepp-cli-hill` (pass)
2. 39-hillslope batch rerun via `target/debug/openwepp-cli-hill` over
   `/tmp/hphys0217_20260531T071120Z/parity/runs/p{1..39}_openwepp.run` (pass)
3. Semantic comparator rerun via:
   `.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
   for `H1..H39` (pass)
4. Semantic summary aggregation (python script over
   `/tmp/hphys0217_20260531T071120Z/parity/reports/semantic/H*.semantic.json`)
   (pass)

## Non-gates by design
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` were not rerun in HPHYS0217
  because package scope is diagnostics-only with no code changes.

## Execution artifacts
- `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_batch_status.tsv`
- `/tmp/hphys0217_20260531T071120Z/parity/reports/semantic_status.tsv`
- `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_semantic_summary.tsv`

## Recovery note
- Initial semantic attempt using system `python3` failed with
  `ModuleNotFoundError: No module named 'pyarrow'`.
- Semantic stage was rerun with `.venv/bin/python` and completed successfully.
