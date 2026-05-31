# HPHYS0223 Gate Results

Status: completed
Evidence mode: Ran

## Commands run
1. `cargo build -p openwepp-runner --bin openwepp-cli-hill` (pass)
2. 39-hillslope batch rerun via `target/debug/openwepp-cli-hill` over
   `/tmp/hphys0223_20260531T201410Z/parity/runs/p{1..39}_openwepp.run` (pass)
3. Semantic comparator first pass with partition filter (invalid settings for
   this lane): produced `common_row_count=0` artifacts (rejected for closure).
4. Semantic comparator rerun with valid settings
   (`--candidate-year-offset 2012`, no partition filter) for `H1..H39` (pass)
5. Summary aggregation over
   `/tmp/hphys0223_20260531T201410Z/parity/reports/semantic/H*.semantic.json`
   (pass)

## Execution artifacts
- `/tmp/hphys0223_20260531T201410Z/parity/reports/hillslope_batch_status.tsv`
- `/tmp/hphys0223_20260531T201410Z/parity/reports/semantic_status.tsv`
- `/tmp/hphys0223_20260531T201410Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0223_20260531T201410Z/parity/reports/hillslope_semantic_summary.tsv`

## Gate decision
- Rerun/readjudication evidence quality: pass.
