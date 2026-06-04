# Implementation Test Evidence

Status: complete
Evidence mode: Ran

## Focused Tests

Ran:
- `cargo test --test hphys0285_spring_soil_storage_retention_contract -- --nocapture`
  - Result: passed, `3 passed`.
- `cargo test --test hphys0283_snowmelt_infiltration_partition_contract --test hphys0284_negative_melt_snowpack_state_contract -- --nocapture`
  - Result: passed, `4 passed` across HPHYS0283/HPHYS0284 adjacent tests.
- `cargo test --test hphys0284_negative_melt_snowpack_state_contract --test hphys0285_spring_soil_storage_retention_contract -- --nocapture`
  - Result: passed, `6 passed` after `CLAUDE-0285-001` remediation.
- `cargo test --test clim05_snow_runtime_kernel_contract --test hphys0283_snowmelt_infiltration_partition_contract -- --nocapture`
  - Result: passed, `10 passed` after `CLAUDE-0285-001` remediation.

## Build And Runtime Smoke

Ran:
- `cargo build --release --package openwepp-runner --bin openwepp-cli-hill`
  - Result: passed.
- H1 release smoke using `/tmp/hphys0285_full_release_final_20260604T201242Z/runs/p1_openwepp.run`
  - Result: passed.
  - Outputs: `/tmp/hphys0285_full_release_final_20260604T201242Z/hillslope_output/H1.wat.parquet`, `/tmp/hphys0285_full_release_final_20260604T201242Z/hillslope_output/H1.loss.json`.
- Post-review H1..H39 release runtime rerun after `CLAUDE-0285-001` remediation:
  - Root: `/tmp/hphys0285_review_remediation_20260604T203602Z`
  - Result: passed, `39/39` return code `0`.

## Full Suite

Ran:
- Full H1..H39 release runtime suite completed: `39/39` return code `0`.
- Full H1..H39 semantic comparator completed: `39/39` reports generated.
- Overall semantic pass remains `0/39`; metrics are recorded in `full-39-suite-metrics.md`.
