# WBVAL05 Validation Ledger

Status: complete

Evidence mode: ran

Purpose: record before/after validation for the four J-95 percolation target
hillslopes.

Required targets:

- `p7`
- `p11`
- `p18`
- `p20`

Required result per target:

| Hillslope | Command | RC | WAT emitted | First failure | Classification | Evidence |
|---|---|---:|---|---|---|---|
| p7 | `target/release/openwepp-cli-hill --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs --run-file /tmp/wbval05_j95_perc_20260606T000000Z/generated_runfiles/p7.toml --output-dir /tmp/wbval05_j95_perc_20260606T000000Z/outputs_final/p7 --policy compat` | 1 | no | `HKERNEL-WB14-RUNOFF-E-003` at sim day 95 | upstream snow/runoff boundary; WB18 PERC cleared | `/tmp/wbval05_j95_perc_20260606T000000Z/final_status.tsv` |
| p11 | same as p7 with `p11.toml` and `outputs_final/p11` | 1 | no | `HKERNEL-WB14-RUNOFF-E-003` at sim day 95 | upstream snow/runoff boundary; WB18 PERC cleared | `/tmp/wbval05_j95_perc_20260606T000000Z/final_status.tsv` |
| p18 | same as p7 with `p18.toml` and `outputs_final/p18` | 1 | no | `HKERNEL-WB14-RUNOFF-E-003` at sim day 95 | upstream snow/runoff boundary; WB18 PERC cleared | `/tmp/wbval05_j95_perc_20260606T000000Z/final_status.tsv` |
| p20 | same as p7 with `p20.toml` and `outputs_final/p20` | 1 | no | `HKERNEL-WB14-RUNOFF-E-003` at sim day 95 | upstream snow/runoff boundary; WB18 PERC cleared | `/tmp/wbval05_j95_perc_20260606T000000Z/final_status.tsv` |

Static:

- Final status no longer contains `HKERNEL-WB11-PERC-E-003`; WBVAL05's
  percolation fail-closed symptom is corrected/reclassified.
- The remaining WAT blocker is not closed in WBVAL05 because it is owned by
  WB14 runoff/snow-domain authority, not percolation.

Ran:

- `cargo fmt --check` passed.
- `cargo test -p openwepp-hillslope-orchestrator
  wbval05_wb18_percolation_consumes_published_zero_infiltration_without_snow_recompute
  -- --nocapture` passed.
- `cargo test -p openwepp-hillslope-orchestrator hphys0246_wb18_percolation
  -- --nocapture` passed.
- `cargo test -p openwepp-hillslope-orchestrator` passed: 102 tests.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` passed.
- Final p7/p11/p18/p20 CLI validation ran and recorded RC 1 with
  `HKERNEL-WB14-RUNOFF-E-003`.
