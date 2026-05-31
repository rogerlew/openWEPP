# HPHYS0219 Gate Results

Status: completed
Evidence mode: Ran

## Commands run
1. `cargo fmt --check` (pass)
2. `cargo clippy --workspace --all-targets -- -D warnings` (pass)
3. `cargo test --workspace` (pass)
4. `cargo deny check` (pass; warnings only: duplicate crates + unmatched
   allowlist license entries)
5. Targeted WB19 tests:
   - `cargo test --test hphys0219_wb19_coca_threshold_contract --test wb19_lateral_drainage_physics_kernel_contract` (pass)
   - `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::` (pass)
6. 39-hillslope rerun via `target/debug/openwepp-cli-hill` over
   `/tmp/hphys0219_20260531T083756Z/parity/runs/p{1..39}_openwepp.run` (pass)
7. Semantic comparator rerun via:
   `.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
   for `H1..H39` with `--candidate-year-offset 2012` (pass)
8. Semantic summary aggregation (python script over
   `/tmp/hphys0219_20260531T083756Z/parity/reports/semantic/H*.semantic.json`)
   (pass)

## Execution artifacts
- `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_batch_status.tsv`
- `/tmp/hphys0219_20260531T083756Z/parity/reports/semantic_status.tsv`
- `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_semantic_summary.tsv`

## Integrity note
- Semantic lane executed with explicit `--candidate-year-offset 2012`; summary
  reports non-zero overlap for all hillslopes (`39/39`).
