# HPHYS0221 Gate Results

Status: completed
Evidence mode: Ran

## Commands run
1. `cargo fmt --check` -> pass
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
3. `cargo test --workspace` -> pass
4. `cargo deny check` -> pass (warning-only duplicate/unmatched-license allow entries)
5. Targeted WB19/fixture suites rerun during stabilization -> pass

## Rerun execution artifacts
- HPHYS0219 summary:
  `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0221 summary:
  `/tmp/hphys0221_20260531T141839Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0221 run status files:
  - `/tmp/hphys0221_20260531T141839Z/parity/reports/hillslope_batch_status.tsv`
  - `/tmp/hphys0221_20260531T141839Z/parity/reports/semantic_status.tsv`

## Gate decision
- `MEASURE-HP221-004`: pass.
