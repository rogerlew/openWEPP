# Gate Results

Status: complete
Evidence mode: Ran

## Contract Gate

- `cargo test --test hphys0288_winter_rain_snowmelt_partition_contract -- --nocapture`
  - Pre-implementation result: failed as expected with missing released-rain routed melt.
  - Final result: passed, 3 tests passed.

## Focused Trace Gates

- `cargo test -p openwepp-runner hphys0288_trace_row_captures_rain_on_snow_release_without_snowpack_loss -- --nocapture`
  - Result: passed.
- `cargo test -p openwepp-runner hphys0245_trace_writer_serializes_jsonl_rows -- --nocapture`
  - Result: passed.

## Authority Anti-Evasion Gates

- `bash tools/release/check_authority_suite_antievasion.sh`
  - Result: passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  - Result: passed, 2 passed.

## Rust Workspace Gates

- `cargo fmt --check`
  - Result: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed.
- `cargo test --workspace`
  - Result: passed.
- `cargo deny check`
  - Result: passed with existing warnings for duplicate `getrandom`, `hashbrown`, `twox-hash` lock entries and unmatched `ISC` / `Unicode-DFS-2016` license allowances.

## Full Semantic Gate

- `python docs/work-packages/20260604-hphys0288-winter-rain-on-snow-melt-partition-magnitude-closure-001/artifacts/hphys0288_diagnostics.py --run-root /tmp/hphys0288_full_release_final_v13_20260604T163204Z`
  - Runtime status: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z/reports/hillslope_batch_status.tsv`
  - Semantic status: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z/reports/semantic_status.tsv`
  - Semantic pass: `0/39`.

## Target Trace Gate

- Release H1/H7/H39 trace captures:
  - Root: `/tmp/hphys0288_target_traces_v13_20260604T162402Z`
  - H1: 24,837 rows; 409 post-WB13 released-rain rows.
  - H7: 24,837 rows; 402 post-WB13 released-rain rows.
  - H39: 24,837 rows; 409 post-WB13 released-rain rows.
